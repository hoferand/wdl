import { io } from "./npm_modules/socket.io-client/dist/socket.io.esm.min.js";

import init, { check_src } from "./wasm/wasm.js";

import * as Output from "./output.js";

let editor = null;

let socket = null;

let response_callback = null;

const target_area = document.getElementById("target-area");
const router_request = document.getElementById("router-request");
const router_wait = document.getElementById("router-wait");
const action_text = document.getElementById("action-text");

window.addEventListener("load", async (_event) => {
	await init();

	require.config({
		paths: { vs: "npm_modules/monaco-editor/min/vs" },
	});

	require(["vs/editor/editor.main"], function () {
		define_wdl();

		editor = monaco.editor.create(document.getElementById("editor-container"), {
			value: default_source,
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
});

document.getElementById("run-btn").addEventListener("click", async (_event) => {
	if (socket) {
		close_socket();
		Output.add_warn("Order canceled by user.");
		return;
	}

	monaco.editor.setModelMarkers(editor.getModel(), "owner", []);

	document.getElementById("run-btn").innerHTML = "Stop";

	Output.clear();
	Output.add_info("Start order.");

	console.log("open socket");
	let proto = "ws://";
	if (window.location.protocol === "https:") {
		proto = "wss://";
	}
	socket = io(proto + window.location.host + "/run", {
		reconnectionDelayMax: 10000,
	});

	socket.on("log", Output.add_log);

	socket.on("router_request", (request, callback) => {
		console.log("Received router request:", request);
		response_callback = callback;
		const action = request.action;
		action_text.innerText = action.charAt(0).toUpperCase() + action.slice(1);
		if (action === "pickup") {
			action_text.innerText += " from:";
		} else {
			action_text.innerText += " to:";
		}
		target_area.innerText = JSON.stringify(
			request.target,
			(_key, value) => {
				if (value !== null) return value;
			},
			4
		);
		router_wait.style.display = "none";
		router_request.style.display = "block";
	});

	socket.on("error", (errors) => {
		errors = JSON.parse(errors);
		print_errors(errors, true);
		close_socket();
		Output.add_warn("Order canceled due to previous error(s).");
	});

	socket.on("done", (pos) => {
		close_socket();
		Output.add_info("Order done.", pos ? { span_str: pos.span_str } : {});
		if (pos) {
			const editor_info = {
				startLineNumber: pos.span.start.line + 1,
				startColumn: pos.span.start.column + 1,
				endLineNumber: pos.span.end.line + 1,
				endColumn: pos.span.end.column + 1,
				message: "Order done.",
				severity: monaco.MarkerSeverity.Info,
			};
			monaco.editor.setModelMarkers(editor.getModel(), "owner", [editor_info]);
		}
	});

	socket.on("canceled", (pos) => {
		close_socket();
		Output.add_warn("Order canceled.", pos ? { span_str: pos.span_str } : {});
		if (pos) {
			const editor_warning = {
				startLineNumber: pos.span.start.line + 1,
				startColumn: pos.span.start.column + 1,
				endLineNumber: pos.span.end.line + 1,
				endColumn: pos.span.end.column + 1,
				message: "Order canceled.",
				severity: monaco.MarkerSeverity.Warning,
			};
			monaco.editor.setModelMarkers(editor.getModel(), "owner", [
				editor_warning,
			]);
		}
	});

	socket.emit("start", editor.getValue());
});

document
	.getElementById("router-done-btn")
	.addEventListener("click", (_event) => {
		hide_router();
		if (response_callback) {
			response_callback("Done");
		} else {
			throw "Response callback not set!";
		}
	});

document
	.getElementById("router-no-station-left-btn")
	.addEventListener("click", (_event) => {
		hide_router();
		if (response_callback) {
			response_callback("NoStationLeft");
		} else {
			throw "Response callback not set!";
		}
	});

function close_socket() {
	hide_router();
	console.log("close socket");
	socket.close();
	socket = null;
	document.getElementById("run-btn").innerHTML = "Start";
}

function hide_router() {
	router_request.style.display = "none";
	router_wait.style.display = "block";
}

const debounced_check = debounce(check, 100);
async function check(src) {
	const status = check_src(src);

	const output = !socket;

	if (status.status === "ok") {
		if (output) {
			Output.clear();
			Output.add_info("No problems found.");
		}
		monaco.editor.setModelMarkers(editor.getModel(), "owner", []);
	} else {
		if (output) {
			Output.clear();
		}
		print_errors(status.errors, output);
	}
}

function print_errors(errors, output) {
	let editor_errors = [];
	for (let error2 of errors) {
		if (output) {
			Output.add_error(
				error2.title,
				error2.pos ? { span_str: error2.pos.span_str } : {}
			);
		}
		if (error2.pos) {
			const pos = error2.pos;

			editor_errors.push({
				startLineNumber: pos.span.start.line + 1,
				startColumn: pos.span.start.column + 1,
				endLineNumber: pos.span.end.line + 1,
				endColumn: pos.span.end.column + 1,
				message: error2.title,
				severity: monaco.MarkerSeverity.Error,
			});
		}
	}
	monaco.editor.setModelMarkers(editor.getModel(), "owner", editor_errors);
}

function debounce(func, timeout = 300) {
	let timer;
	return (...args) => {
		clearTimeout(timer);
		timer = setTimeout(() => {
			func.apply(this, args);
		}, timeout);
	};
}

const default_source = `global source = "mySource";
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
			"!",
			"+",
			"-",
			"*",
			"/",
			"%",
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
