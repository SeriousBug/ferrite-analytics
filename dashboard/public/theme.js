(() => {
  if (JSON.parse(localStorage.getItem("Theme")) === "dark") {
    document.body.dataset.theme = "forest";
  } else {
    document.body.dataset.theme = "autumn";
  }
})();
