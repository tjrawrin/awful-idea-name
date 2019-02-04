var el = document.getElementById('project-name');
var btn = document.getElementById('next-button');

btn.addEventListener('click', function () {
  el.textContent = wasm_bindgen.generate_name();
});

wasm_bindgen('./lousy_project_name_bg.wasm')
  .then(function () {
    el.textContent = wasm_bindgen.generate_name();
  });
