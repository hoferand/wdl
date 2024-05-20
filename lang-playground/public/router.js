/**
 * ResponseCallback
 *
 * @callback ResponseCallback
 *
 * @param {string} status
 *
 * @returns {void}
 */

import * as Output from "./output.js";

const ROUTER_REQUEST = document.getElementById("router-request");
const ROUTER_WAIT = document.getElementById("router-wait");
const ACTION_TEXT = document.getElementById("action-text");
const TARGET_AREA = document.getElementById("target-area");

/**
 * Saves the response callback for the router status.
 *
 * @type {ResponseCallback|null}
 */
let response_callback = null;

/**
 * Displays a new request.
 *
 * @param {{action: "Pickup"|"Drop"|"Drive", target: any}} request
 * @param {ResponseCallback} callback
 *
 * @returns {void}
 */
export function set_request(request, callback) {
	response_callback = callback;

	const action = request.action;
	ACTION_TEXT.innerText = action.charAt(0).toUpperCase() + action.slice(1);
	if (action === "Pickup") {
		ACTION_TEXT.innerText += " from:";
	} else if (["Drop", "Drive"].includes(action)) {
		ACTION_TEXT.innerText += " to:";
	} else {
		Output.add_error(`Received invalid action type \`${action}\`!`);
		throw `Received invalid action type \`${action}\`!`;
	}

	TARGET_AREA.innerText = JSON.stringify(
		request.target,
		(_key, value) => (value !== null ? value : undefined),
		4
	);

	show_request();
}

/**
 * Cancels the current request.
 *
 * @returns {void}
 */
export function cancel_request() {
	response_callback = null;
	hide_request();
}

/**
 * Shows all request HTML elements.
 *
 * @returns {void}
 */
function show_request() {
	ROUTER_WAIT.style.display = "none";
	ROUTER_REQUEST.style.display = "block";
}

/**
 * Hides all request HTML elements.
 *
 * @returns {void}
 */
function hide_request() {
	ROUTER_REQUEST.style.display = "none";
	ROUTER_WAIT.style.display = "block";
}

/**
 * Sends router status "Done" to the interpreter.
 *
 * @returns {void}
 */
function send_done() {
	send_status("Done");
}

/**
 * Sends router status "NoStationLeft" to the interpreter.
 *
 * @returns {void}
 */
function send_no_station_left() {
	send_status("NoStationLeft");
}

/**
 * Sends a status to the current interpreter connection.
 *
 * @param {string} status
 *
 * @returns {void}
 */
function send_status(status) {
	hide_request();
	if (!response_callback) {
		Output.add_error("Failed to send router status!");
		throw "Router response callback not set!";
	}
	response_callback(status);
	response_callback = null;
}

/**
 * Event listeners for the status buttons.
 */

document.getElementById("router-done-btn").addEventListener("click", send_done);

document
	.getElementById("router-no-station-left-btn")
	.addEventListener("click", send_no_station_left);
