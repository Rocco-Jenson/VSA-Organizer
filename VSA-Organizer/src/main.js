const { invoke } = window.__TAURI__.tauri;

let vsaWriteVarTextOutput; // For fTextOutputorm input & output
let vsaReadVarTextOutput;
let vsaReadVar; // For reading file
let vsaQueryVarTextOutput;
let vsaQueryVar;

async function vsaWriteRust(name, date, cost) {
    vsaWriteVarTextOutput.value = await invoke("vsa_write", { name: name.value, date: date.value, cost: cost.value})
    setTimeout(() => {
        vsaWriteVarTextOutput.value = "";
    }, 3000);
}

async function vsaReadRust() {
    vsaReadVar.value = await invoke("vsa_read", {})
}

async function vsaRemoveRust() {
    vsaReadVarTextOutput.value = await invoke("vsa_remove", {})
    setTimeout(() => {
        vsaReadVarTextOutput.value = "";
    }, 3000);
}

async function vsaQueryRust(query) {
    vsaQueryVar.value = await invoke("vsa_query", { query: query.value })
}

window.addEventListener("DOMContentLoaded", () => {
    /* vsa_write.html Finished */
    if (window.location.pathname === "/vsa_write.html") {
        vsaWriteVarTextOutput = document.querySelector("#form-output")
        document.querySelector(".input-form").addEventListener("submit", (event) => {
            event.preventDefault();
            let name = document.querySelector("#name-field");
            let date = document.querySelector("#date-field");
            let cost = document.querySelector("#cost-field");
            vsaWriteRust(name, date, cost);
            document.querySelector("#vsa-form").reset();
        });
    }
    /* vsa_read.html Finished */
    if (window.location.pathname === "/vsa_read.html") {
        vsaReadVar = document.querySelector("#textarea-output");
        setInterval(vsaReadRust, 500);

        document.querySelector("#remove-class").addEventListener("click", () => {
            vsaReadVarTextOutput = document.querySelector("#read-output");
            vsaRemoveRust();
        });
    }

    if (window.location.pathname === "/vsa_query.html") {
        vsaQueryVar = document.querySelector("#query-textarea-output");
        document.querySelector(".input-form").addEventListener("submit", (event) => {
            event.preventDefault();
            let query = document.querySelector("#query-field");
            vsaQueryRust(query);
            document.querySelector("#query-form").reset();
        });
    }
});
