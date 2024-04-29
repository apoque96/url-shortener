const $valid = document.getElementById("valid_url");
const $long = document.getElementById("long");
const $short = document.getElementById("short");

if (!$valid.textContent || !$long.textContent) {
  $long.parentElement.classList.add("hidden");
  $short.parentElement.classList.add("hidden");
  if ($valid.textContent == "false") {
    alert("Invalid link");
    $valid.textContent = "true";
  }
} else {
  $long.parentElement.classList.remove("hidden");
  $short.parentElement.classList.remove("hidden");
}
