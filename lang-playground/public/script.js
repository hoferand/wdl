import init, { check_src } from "./wasm/wasm.js";
import { io } from "./npm_modules/socket.io-client/dist/socket.io.esm.min.js";

let editor = null;

let socket = null;

let response_callback = null;

const output_area = document.getElementById("output-area");
const target_area = document.getElementById("target-area");
const router_request = document.getElementById("router-request");
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
		router_request.style.display = "none";
		close_socket();
		output_area.innerHTML += info("Order canceled by user.\n");
		return;
	}

	monaco.editor.setModelMarkers(editor.getModel(), "owner", []);

	document.getElementById("run-btn").innerHTML = "Stop";

	output_area.innerHTML = info("Start order.\n");

	console.log("open socket");
	let proto = "ws://";
	if (window.location.protocol === "https:") {
		proto = "wss://";
	}
	socket = io(proto + window.location.host + "/run", {
		reconnectionDelayMax: 10000,
	});

	socket.on("log", (log) => {
		if (log.level == "Info") {
			output_area.innerHTML += info(html_escape(log.msg)) + "\n";
		} else if (log.level == "Warn") {
			output_area.innerHTML += warn(html_escape(log.msg)) + "\n";
		} else if (log.level == "Error") {
			output_area.innerHTML += error(html_escape(log.msg)) + "\n";
		} else {
			throw "Invalid log level received!";
		}
	});

	socket.on("router_request", (request, callback) => {
		console.log("Received router request:", request);
		response_callback = callback;
		const action = request.action;
		action_text.innerText =
			action.charAt(0).toUpperCase() + action.slice(1) + " from:";
		target_area.innerText = JSON.stringify(
			request.target,
			(_key, value) => {
				if (value !== null) return value;
			},
			4
		);
		router_request.style.display = "block";
	});

	socket.on("error", (errors) => {
		errors = JSON.parse(errors);
		print_errors(errors, output_area);
		close_socket();
		output_area.innerHTML += error(
			"Order canceled due to previous error(s).\n"
		);
	});

	socket.on("done", (pos) => {
		close_socket();
		output_area.innerHTML += info("Order done.\n");
		if (pos) {
			output_area.innerHTML += pos.span_str + "\n";
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
		output_area.innerHTML += warn("Order canceled.\n");
		if (pos) {
			output_area.innerHTML += pos.span_str + "\n";
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

document.getElementById("done-btn").addEventListener("click", (_event) => {
	router_request.style.display = "none";
	if (response_callback) {
		response_callback("Done");
	} else {
		throw "Response callback not set!";
	}
});

document
	.getElementById("no-station-left-btn")
	.addEventListener("click", (_event) => {
		router_request.style.display = "none";
		if (response_callback) {
			response_callback("NoStationLeft");
		} else {
			throw "Response callback not set!";
		}
	});

function close_socket() {
	console.log("close socket");
	socket.close();
	socket = null;
	document.getElementById("run-btn").innerHTML = "Start";
}

function info(msg) {
	if (msg[0] == "[") {
		return '<span class="blue">[INFO]</span>' + msg;
	}
	return '<span class="blue">[INFO]</span>: ' + msg;
}

function warn(msg) {
	if (msg[0] == "[") {
		return '<span class="orange">[WARN]</span>' + msg;
	}
	return '<span class="orange">[WARN]</span>: ' + msg;
}

function error(msg) {
	if (msg[0] == "[") {
		return '<span class="red">[ERROR]</span>' + msg;
	}
	return '<span class="red">[ERROR]</span>: ' + msg;
}

const debounced_check = debounce(check, 100);
async function check(src) {
	let status = check_src(src);

	let output = {};
	if (!socket) {
		output = output_area;
	}

	if (status.status === "ok") {
		output.innerHTML = info("No problems found");
		monaco.editor.setModelMarkers(editor.getModel(), "owner", []);
	} else {
		output.innerHTML = "";
		print_errors(status.errors, output);
	}
}

function print_errors(errors, output) {
	let editor_errors = [];
	for (let error2 of errors) {
		output.innerHTML += error(html_escape(error2.title)) + "\n";
		if (error2.pos) {
			const pos = error2.pos;
			output.innerHTML += pos.span_str + "\n";

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

function html_escape(str) {
	return str.replace(
		/[\u00A0-\u9999<>\&]/g,
		(i) => "&#" + i.charCodeAt(0) + ";"
	);
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
			"par",
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
		],
	});
}
