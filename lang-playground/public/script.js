import init, { check_src } from "./wasm/wasm.js";

var editor = null;

window.addEventListener("load", async (event) => {
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

const debounced_check = debounce(check, 100);
async function check(src) {
	let status = check_src(src);

	const output = document.getElementById("output-area");

	let errors = [];
	if (status.status === "ok") {
		output.innerHTML = "No problems found";
	} else {
		output.innerHTML = "";
		for (let error of status.errors) {
			output.innerHTML += '<span class="red">ERROR:</span> ';
			output.innerHTML += html_escape(error);
			if (error.pos) {
				const pos = error.pos;
				output.innerHTML += "\n" + pos.span_str + "\n";

				errors.push({
					startLineNumber: pos.span.start.line + 1,
					startColumn: pos.span.start.column + 1,
					endLineNumber: pos.span.end.line + 1,
					endColumn: pos.span.end.column + 1,
					message: error.title,
					severity: monaco.MarkerSeverity.Error,
				});
			}
		}
	}
	monaco.editor.setModelMarkers(editor.getModel(), "owner", errors);
}

function html_escape(str) {
	return str.title.replace(
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
