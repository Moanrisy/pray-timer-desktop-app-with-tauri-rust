const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
    greetMsgEl = document.querySelector("#greet-msg");
    const greeting = await invoke("greet");

    const formattedGreeting = greeting.replace(/\n/g, "<br>");

    const startIndex = formattedGreeting.indexOf("C:");
    const truncatedGreeting = formattedGreeting.substring(startIndex);

    greetMsgEl.innerHTML = truncatedGreeting;

    greetMsgEl.setAttribute("title", greeting);
}


function fn60sec() {
  greet();
}

fn60sec();

setInterval(fn60sec, 60 * 1000); // 60 * 1000 milsec
