/**
 * Location
 *
 * @typedef {{line: number, column: number}} Location
 */

/**
 * Span
 *
 * @typedef {{start: Location, end: Location}} Span
 */

/**
 * Log
 *
 * `msg` is not html escaped
 * `span_str` is html escaped
 *
 * @typedef {{level: "Trace"|"Info"|"Warn"|"Error", msg: string, user?: boolean, span?: Span, span_str?: string}} Log
 */

const output_area = document.getElementById("output-area");

const TRACE = '<span class="green">[TRACE]</span>';
const INFO = '<span class="blue">[INFO]</span>';
const WARN = '<span class="orange">[WARN]</span>';
const ERROR = '<span class="red">[ERROR]</span>';

/**
 * Clears the output field.
 *
 * @returns {void}
 */
export function clear() {
	output_area.innerHTML = "";
}

/**
 * Adds a log to the output field.
 *
 * @param {Log} log
 *
 * @returns {void}
 */
export function add_log(log) {
	output_area.innerHTML += format_log(log);
}

/**
 * Adds a trace message to the output field.
 *
 * @param {string} msg
 * @param {{span?: Span, span_str?: string}} pos
 *
 * @returns {void}
 */
export function add_trace(msg, pos = {}) {
	add_log({ level: "Trace", msg, ...pos });
}

/**
 * Adds a info message to the output field.
 *
 * @param {string} msg
 * @param {{span?: Span, span_str?: string}} pos
 *
 * @returns {void}
 */
export function add_info(msg, pos = {}) {
	add_log({ level: "Info", msg, ...pos });
}

/**
 * Adds a warn message to the output field.
 *
 * @param {string} msg
 * @param {{span?: Span, span_str?: string}} pos
 *
 * @returns {void}
 */
export function add_warn(msg, pos = {}) {
	add_log({ level: "Warn", msg, ...pos });
}

/**
 * Adds a error message to the output field.
 *
 * @param {string} msg
 * @param {{span?: Span, span_str?: string}} pos
 *
 * @returns {void}
 */
export function add_error(msg, pos = {}) {
	add_log({ level: "Error", msg, ...pos });
}

/**
 * Formats a log to a string representation.
 *
 * @param {Log} log
 *
 * @returns {string}
 */
function format_log(log) {
	let ret = "";

	if (log.level === "Trace") {
		ret += TRACE;
	} else if (log.level === "Info") {
		ret += INFO;
	} else if (log.level === "Warn") {
		ret += WARN;
	} else if (log.level === "Error") {
		ret += ERROR;
	} else {
		throw `Invalid log level \`${log.level}\`!`;
	}

	if (log.span !== undefined) {
		ret += `[${log.span.start.line + 1}:${log.span.start.column + 1}]`;
	}

	if (log.user ?? false) {
		ret += "[user]";
	}

	ret += ": ";
	ret += html_escape(log.msg);
	if (log.msg.slice(-1) !== "\n") {
		ret += "\n";
	}

	if (log.span_str !== undefined) {
		ret += log.span_str;
		if (log.span_str.slice(-1) !== "\n") {
			ret += "\n";
		}
	}

	return ret;
}

/**
 * Escapes strings to be html secure.
 *
 * @param {string} str
 * @returns string
 */
function html_escape(str) {
	return str.replace(
		/[\u00A0-\u9999<>\&]/g,
		(i) => "&#" + i.charCodeAt(0) + ";"
	);
}
