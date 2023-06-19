const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
    greetMsgEl = document.querySelector("#greet-msg");
    const greeting = await invoke("greet");

    console.log("is looping?" + greeting);

    const formattedGreeting = greeting.replace(/\n/g, "<br>");

    const startIndex = formattedGreeting.indexOf("C:");
    const truncatedGreeting = formattedGreeting.substring(startIndex);

    greetMsgEl.innerHTML = truncatedGreeting;

    greetMsgEl.setAttribute("title", greeting);
}


function fn60sec() {
  console.log("should run every 60s");
  greet();
}

fn60sec();

setInterval(fn60sec, 5 * 1000); // 60 * 1000 milsec
