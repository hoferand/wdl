import "./typedef.js";
import * as Output from "./output.js";

const routerRequest = document.getElementById("router-request");
const routerWait = document.getElementById("router-wait");
const actionText = document.getElementById("action-text");
const targetArea = document.getElementById("target-area");

/**
 * Saves the response callback for the router status.
 *
 * @type {ResponseCallback|null}
 */
let responseCallback = null;

/**
 * Displays a new request.
 *
 * @param {{action: "Pickup"|"Drop"|"Drive", target: any}} request
 * @param {ResponseCallback} callback
 * @returns {void}
 */
export function set_request(request, callback) {
	responseCallback = callback;

	const action = request.action;
	actionText.innerText = action.charAt(0).toUpperCase() + action.slice(1);
	if (action === "Pickup") {
		actionText.innerText += " from:";
	} else if (["Drop", "Drive"].includes(action)) {
		actionText.innerText += " to:";
	} else {
		Output.add_error(`Received invalid action type \`${action}\`!`);
		throw `Received invalid action type \`${action}\`!`;
	}

	targetArea.innerText = JSON.stringify(
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
	responseCallback = null;
	hide_request();
}

/**
 * Shows all request HTML elements.
 *
 * @returns {void}
 */
function show_request() {
	routerWait.style.display = "none";
	routerRequest.style.display = "block";
}

/**
 * Hides all request HTML elements.
 *
 * @returns {void}
 */
function hide_request() {
	routerRequest.style.display = "none";
	routerWait.style.display = "block";
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
 * @returns {void}
 */
function send_status(status) {
	hide_request();
	if (!responseCallback) {
		Output.add_error("Failed to send router status!");
		throw "Router response callback not set!";
	}
	responseCallback([status]);
	responseCallback = null;
}

/**
 * Event listeners for the status buttons.
 */

document.getElementById("router-done-btn").addEventListener("click", send_done);

document
	.getElementById("router-no-station-left-btn")
	.addEventListener("click", send_no_station_left);
