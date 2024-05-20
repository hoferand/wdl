import { io } from "./npm_modules/socket.io-client/dist/socket.io.esm.min.js";

import "./typedef.js";
import * as Editor from "./editor.js";
import * as Router from "./router.js";
import * as Output from "./output.js";

const RUN_BTN = document.getElementById("run-btn");

/**
 * Holds the current Socket.IO socket.
 */
let socket = null;

Editor.init(document.getElementById("editor-container"));

RUN_BTN.addEventListener("click", async (_event) => {
	if (socket) {
		close_socket();
		Output.add_warn("Order canceled by user.");
		return;
	}

	Output.clear();
	Output.add_info("Start order.");

	Editor.clear_markers();
	Editor.disable_output();

	let proto = "ws://";
	if (window.location.protocol === "https:") {
		proto = "wss://";
	}
	socket = io(proto + window.location.host + "/run", {
		reconnectionDelayMax: 10_000,
	});

	RUN_BTN.innerText = "Stop";

	socket.on("log", Output.add_log);

	socket.on("router_request", Router.set_request);

	socket.on("error", (/** @type {string} err_str */ err_str) => {
		close_socket();
		/** @type {WdlError[]} errors */
		const errors = JSON.parse(err_str);
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
	RUN_BTN.innerText = "Start";
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
	let editor_errors = [];
	for (let error2 of errors) {
		Output.add_error(
			error2.title,
			error2.pos ? { span_str: error2.pos.span_str } : {}
		);
		if (error2.pos) {
			editor_errors.push({
				severity: "Error",
				message: error2.title,
				span: error2.pos.span,
			});
		}
	}
	Editor.set_markers(editor_errors);
}
