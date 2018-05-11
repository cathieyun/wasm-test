const js_hello_world = import("./js_hello_world");

const submitButton = document.getElementById("submitButton");

submitButton.addEventListener("click", event => {
	js_hello_world.then(js => {
	  js.get_value(document.getElementById("inputValue").value);
	});
});
