const js = import("./js_hello_world");

const submitButton = document.getElementById("submitButton");

submitButton.addEventListener("click", event => {
	js.then(js => {
	  js.get_value(document.getElementById("inputValue").value);
	});
});
