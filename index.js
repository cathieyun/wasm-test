const bulletproofs_wasm = import("./bulletproofs_wasm");

const submitButton = document.getElementById("submitButton");

submitButton.addEventListener("click", event => {
	bulletproofs_wasm.then(js => {
	  js.get_value(document.getElementById("inputValue").value);
	});
});
