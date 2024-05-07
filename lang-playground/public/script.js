import init, { check_src } from "./wasm/wasm.js";
import { io } from "./npm_modules/socket.io-client/dist/socket.io.esm.min.js";

let editor = null;

let socket = null;

const output_area = document.getElementById("output-area");

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
		output_area.innerHTML += info("Order canceled by user.\n");
		return;
	}

	document.getElementById("run-btn").innerHTML = "Stop";

	output_area.innerHTML = info("Start order.\n");

	console.log("open socket");
	socket = io("ws://localhost:3000/run", {
		reconnectionDelayMax: 10000,
	});

	socket.on("log", (log) => {
		if (log.level == "info") {
			output_area.innerHTML += info(html_escape(log.msg));
		} else if (log.level == "warn") {
			output_area.innerHTML += warn(html_escape(log.msg));
		} else if (log.level == "error") {
			output_area.innerHTML += error(html_escape(log.msg));
		}
	});

	socket.on("router_request", (request) => {
		// TODO: show buttons and send request
		console.log(request);
		socket.emit("router_response", "Done");
		//socket.emit("router_response", "NoStationLeft");
	});

	socket.on("error", (errors) => {
		errors = JSON.parse(errors);
		print_errors(errors, output_area);
		close_socket();
		output_area.innerHTML += error("Order canceled due to error(s).\n");
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
			const editor_warn = {
				startLineNumber: pos.span.start.line + 1,
				startColumn: pos.span.start.column + 1,
				endLineNumber: pos.span.end.line + 1,
				endColumn: pos.span.end.column + 1,
				message: "Order canceled.",
				severity: monaco.MarkerSeverity.Warning,
			};
			monaco.editor.setModelMarkers(editor.getModel(), "owner", [editor_warn]);
		}
	});

	socket.emit("start", editor.getValue());
});

function close_socket() {
	console.log("close socket");
	socket.close();
	socket = null;
	document.getElementById("run-btn").innerHTML = "Start";
}

function info(msg) {
	return '<span class="blue">[INFO]</span>: ' + msg;
}

function warn(msg) {
	return '<span class="orange">[WARN]</span>: ' + msg;
}

function error(msg) {
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
		output.innerHTML += error(html_escape(error2.title));
		if (error2.pos) {
			const pos = error2.pos;
			output.innerHTML += "\n" + pos.span_str + "\n";

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