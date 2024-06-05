import wasm_init, { check_src } from "./wasm/wasm.js";
await wasm_init();

import "./typedef.js";
import * as Output from "./output.js";

/**
 * This source code is set on editor load.
 */
const DEFAULT_SOURCE_CODE = `global source = "mySource";
global destination = "myDestination";

actions {
    log::info("Order started.");

    action::pickup(
        target: {
            stations: [
                source
            ]
        },
        events: {
            no_station_left: order::cancel
        }
    );

    action::drop(
        target: {
            stations: [
                destination
            ]
        }
    );
}
`;

/**
 * The monaco editor instance.
 */
let editor = null;

/**
 * Indicates if errors should be
 * printed to the output field.
 */
let output = true;

/**
 * Initializes the monaco editor inside `container`.
 *
 * @param {HTMLDivElement} container
 * @returns {void}
 */
export function init(container) {
	require.config({
		paths: { vs: "npm_modules/monaco-editor/min/vs" },
	});

	require(["vs/editor/editor.main"], function () {
		define_wdl();

		editor = monaco.editor.create(container, {
			value: DEFAULT_SOURCE_CODE,
			language: "wdl",
			theme: "wdl-theme",
			minimap: { enabled: false },
		});

		editor.getModel().updateOptions({ tabSize: 4 });

		editor.getModel().onDidChangeContent((_event) => {
			debounced_check(editor.getValue());
		});

		check(editor.getValue());
	});
}

/**
 * Enables logging to the output field.
 *
 * @returns {void}
 */
export function enable_output() {
	output = true;
}

/**
 * Disables logging to the output field.
 *
 * @returns {void}
 */
export function disable_output() {
	output = false;
}

/**
 * Sets markers to the editor.
 * Previous markers gets deleted.
 *
 * @param {Marker[]} markers
 * @returns {void}
 */
export function set_markers(markers) {
	let converted = [];

	for (const marker of markers) {
		converted.push({
			severity: convert_severity(marker.severity),
			message: marker.message,
			startLineNumber: marker.span.start.line + 1,
			startColumn: marker.span.start.column + 1,
			endLineNumber: marker.span.end.line + 1,
			endColumn: marker.span.end.column + 1,
		});
	}

	monaco.editor.setModelMarkers(editor.getModel(), "owner", converted);
}

/**
 * Clears all markers from the editor.
 *
 * @returns {void}
 */
export function clear_markers() {
	monaco.editor.setModelMarkers(editor.getModel(), "owner", []);
}

/**
 * Returns the current source code.
 *
 * @returns {string}
 */
export function get_code() {
	return editor.getValue();
}

/**
 * Converts the severity enum to the corresponding number.
 *
 * @param {Severity} severity
 * @returns {number}
 */
function convert_severity(severity) {
	switch (severity) {
		case "Hint":
			return monaco.MarkerSeverity.Hint;
		case "Info":
			return monaco.MarkerSeverity.Info;
		case "Warning":
			return monaco.MarkerSeverity.Warning;
		case "Error":
			return monaco.MarkerSeverity.Error;
		default:
			if (output) {
				Output.add_error("Internal error!");
			}
			throw `Received invalid severity \`${severity}\`!`;
	}
}

/**
 * Checks if the given source code is syntactically correct.
 *
 * @param {string} src
 * @returns {void}
 */
function check(src) {
	const status = check_src(src);

	if (output) {
		Output.clear();
	}

	if (status.status === "Ok") {
		if (output) {
			Output.add_info("No problems found.");
		}
		clear_markers();
	} else if (status.status === "Error") {
		display_errors(status.errors, output);
	} else {
		if (output) {
			Output.add_error("Received invalid status from source code checker!");
		}
		clear_markers();
		throw `Received invalid status \`${status.status}\` from source code checker!`;
	}
}

/**
 * Debounced version of `check()`.
 *
 * @param {string} src
 * @returns {void}
 */
const debounced_check = debounce(check, 100);

/**
 * Displays the given errors inline in the editor (if position is available)
 * and prints them to the output field if enabled.
 *
 * @param {WdlError[]} errors
 * @returns {void}
 */
function display_errors(errors) {
	let editor_errors = [];
	for (let error2 of errors) {
		if (output) {
			Output.add_error(
				error2.title,
				error2.pos ? { span_str: error2.pos.span_str } : {}
			);
		}
		if (error2.pos) {
			editor_errors.push({
				severity: "Error",
				message: error2.title,
				span: error2.pos.span,
			});
		}
	}
	set_markers(editor_errors);
}

/**
 * Returns a debounced version of `func`.
 *
 * @param {function(...any): any} func
 * @param {number} timeout
 * @returns {function(...any): any}
 */
function debounce(func, timeout = 300) {
	let timer;
	return (...args) => {
		clearTimeout(timer);
		timer = setTimeout(() => {
			func.apply(this, args);
		}, timeout);
	};
}

/**
 * Defines WDL as language for the monaco editor.
 *
 * @returns {void}
 */
function define_wdl() {
	monaco.languages.register({
		id: "wdl",
		extensions: [".wdl"],
		aliases: ["WDL", "wdl"],
		mimetypes: ["application/wdl"],
	});

	monaco.languages.setMonarchTokensProvider("wdl", {
		brackets: [
			{ open: "{", close: "}", token: "delimiter.curly" },
			{ open: "[", close: "]", token: "delimiter.bracket" },
			{ open: "(", close: ")", token: "delimiter.parenthesis" },
		],
		ignoreCase: false,
		unicode: true,
		keywordsControl: [
			"actions",
			"if",
			"else",
			"while",
			"continue",
			"break",
			"return",
			"spawn",
		],
		keywordsOther: ["global", "function", "let", "true", "false", "null"],
		operators: [
			"+",
			"-",
			"*",
			"/",
			"%",
			"??",
			"!",
			"<",
			">",
			"==",
			"!=",
			"<=",
			">=",
			"and",
			"or",
		],
		tokenizer: {
			root: [
				[/(\p{XID_Start}|_)\p{XID_Continue}*(?=::)/, "support.class"],
				[/(\p{XID_Start}|_)\p{XID_Continue}*(?=\()/, "support.function"],
				[/\d+(\.\d+)?/, "number"],
				[
					/(\p{XID_Start}|_)\p{XID_Continue}*\b/,
					{
						cases: {
							"@keywordsControl": "keyword.control",
							"@keywordsOther": "keyword.other",
							"@operators": "operator",
							"@default": "variable",
						},
					},
				],
				{ include: "@comment" },
				{ include: "@string" },
			],
			comment: [
				[/\/\/.*$/, "comment"],
				[/\/\*/, "comment", "@comment_body"],
			],
			comment_body: [
				[/\*\//, "comment", "@pop"],
				[/./, "comment"],
			],
			string: [[/"/, "string", "@string_body"]],
			string_body: [
				[/"/, "string", "@pop"],
				[/(\\n|\\"|\\\\)/, "string.escape"],
				[/\\./, "string.escape.invalid"],
				[/./, "string"],
			],
		},
	});

	monaco.languages.setLanguageConfiguration("wdl", {
		comments: { lineComment: "//", blockComment: ["/*", "*/"] },
		brackets: [
			["{", "}"],
			["[", "]"],
			["(", ")"],
		],
		surroundingPairs: [
			{ open: "{", close: "}" },
			{ open: "[", close: "]" },
			{ open: "(", close: ")" },
			{ open: '"', close: '"' },
		],
		autoClosingPairs: [
			{ open: "{", close: "}" },
			{ open: "[", close: "]" },
			{ open: "(", close: ")" },
			{ open: '"', close: '"', notIn: ["string", "comment"] },
			{ open: "/*", close: "*/", notIn: ["string", "comment"] },
		],
	});

	monaco.editor.defineTheme("wdl-theme", {
		base: "vs",
		inherit: true,
		colors: {},
		rules: [
			{
				token: "keyword.control",
				foreground: "#AF00DB",
			},
			{
				token: "support.class",
				foreground: "#267F99",
			},
			{
				token: "support.function",
				foreground: "#795d26",
			},
			{
				token: "string.escape",
				foreground: "#ee0000",
			},
		],
	});
}
