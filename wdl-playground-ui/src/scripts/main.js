import { io } from "socket.io-client";

import "./typedef.js";
import * as Editor from "./editor.js";
import * as Router from "./router.js";
import * as Output from "./output.js";

const runButton = document.getElementById("run-btn");

/**
 * Holds the current Socket.IO socket.
 */
let socket = null;

Editor.init(document.getElementById("editor-container"));

runButton.addEventListener("click", () => {
	if (socket) {
		close_socket();
		Output.add_warn("Order canceled by user.");
		return;
	}

	Output.clear();
	Output.add_info("Start order.");

	Editor.clear_markers();
	Editor.disable_output();

	socket = io("/run", {
		reconnectionDelayMax: 10_000,
	});

	runButton.innerText = "Stop";

	socket.on("log", Output.add_log);

	socket.on("router_request", Router.set_request);

	socket.on("error", (/** @type {WdlError[]} errors */ errors) => {
		close_socket();
		display_errors(errors);
		Output.add_warn("Order canceled due to previous error(s).");
	});

	socket.on("done", (/** @type {Position?} pos */ pos) => {
		close_socket();
		Output.add_info("Order done.", pos ? { span_str: pos.span_str } : {});
		if (pos) {
			Editor.set_markers([
				{
					severity: "Info",
					message: "Order done.",
					span: pos.span,
				},
			]);
		}
	});

	socket.on("canceled", (/** @type {Position?} pos */ pos) => {
		close_socket();
		Output.add_warn("Order canceled.", pos ? { span_str: pos.span_str } : {});
		if (pos) {
			Editor.set_markers([
				{
					severity: "Warning",
					message: "Order canceled.",
					span: pos.span,
				},
			]);
		}
	});

	socket.emit("start", Editor.get_code());
});

/**
 * Closes the current Socket.IO connection and do some cleanups.
 *
 * @returns {void}
 */
function close_socket() {
	Router.cancel_request();
	socket.close();
	socket = null;
	runButton.innerText = "Start";
	Editor.enable_output();
}

/**
 * Prints the given errors to the output field
 * and displays them inline in the editor (if position is available).
 *
 * @param {WdlError[]} errors
 * @returns {void}
 */
function display_errors(errors) {
	let editorErrors = [];
	for (let error2 of errors) {
		Output.add_error(
			error2.title,
			error2.pos ? { span_str: error2.pos.span_str } : {}
		);
		if (error2.pos) {
			editorErrors.push({
				severity: "Error",
				message: error2.title,
				span: error2.pos.span,
			});
		}
	}
	Editor.set_markers(editorErrors);
}
