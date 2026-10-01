"use strict";

if(!document.body.classList || !document.querySelector) {
    // if our prefs code won't work, don't show the gear
    throw new Error("Your browser is too old. You can't set ye preferences.");
}

let topLevel;
if(document.URL) {
    topLevel = document.URL.match(/(.*aaronfreed\.github\.io)/)[1];
}
if(!topLevel) {
    topLevel = "/";
}

let forcedElements = {};

function forceElement(name, css) {
    if(forcedElements[name]) {
        document.head.removeChild(forcedElements[name]);
        delete forcedElements[name];
    }
    if(css != null) {
        forcedElements[name] = document.createElement("LINK");
        forcedElements[name].setAttribute("rel", "stylesheet");
        forcedElements[name].setAttribute("type", "text/css");
        forcedElements[name].setAttribute("href", `${topLevel}/${css}`);
        document.head.appendChild(forcedElements[name]);
    }
}

function applyPreferences() {
    let cookie = document.cookie;
    if(!cookie) return;
    if(cookie.includes("forceDark")) {
        forceElement("colorScheme", "force_dark.css");
    } else if(cookie.includes("forceLight")) {
        forceElement("colorScheme", "force_light.css");
    } else {
        forceElement("colorScheme", null);
    }
    if(cookie.includes("lysdexicsUntie")) {
        forceElement("font", "lysdexics_untie.css");
    } else {
        forceElement("font", null);
    }
}

function updatePreferences() {
    let elements = [];
    let e;
    if((e = document.querySelector(`input[name="colorScheme"]:checked`)?.value)) {
        elements.push(e);
    }
    if((e = document.querySelector(`input[name="font"]:checked`)?.value)) {
        elements.push(e);
    }
    document.cookie = `preferences=${elements.join("+")}`;
    applyPreferences();
}

// So, that's how we achieve the preferences. But how do we set them?
// Control-I, `document.cookie = "your preferences here"`? NO!

let gear = document.createElement("div");
gear.setAttribute("class", "preferencesButton");
gear.setAttribute("title", "Click here to change font and color scheme preferences.");
gear.addEventListener("click", openPreferences);
gear.innerText = "\u2699";
document.body.appendChild(gear);

let topLevelPrefsElement;

function openPreferences() {
    if(!topLevelPrefsElement) {
        topLevelPrefsElement = document.createElement("DIV");
        topLevelPrefsElement.setAttribute("class", "prefsBox");
        topLevelPrefsElement.innerHTML = `
<p>Color:</p>
<ul>
<li><input type="radio" name="colorScheme" id="colorNeutral" value="" checked> <label for="colorNeutral">Automatic</label> (default)</li>
<li><input type="radio" name="colorScheme" id="colorLight" value="forceLight"> <label for="colorLight">Light</label></li>
<li><input type="radio" name="colorScheme" id="colorDark" value="forceDark"> <label for="colorDark">Dark</label></li>
</ul>
<p>Font:</p>
<ul>
<li class="fira"><input type="radio" name="font" id="fontFira" value="" checked> <label for="fontFira">FiraGO</label> (default)</li>
<li class="atkinson"><input type="radio" name="font" id="fontLysdexia" value="lysdexicsUntie"> <label for="fontLysdexia">Atkinson Hyperlegible Next</label></li>
</ul>
<p><a href="https://aaronfreed.github.io/cookienotice.html" target="_blank">Notice on cookie usage</a></p>
<button id="closePrefs">Close</button>
`;
        let cookie = document.cookie;
        forEachChild(topLevelPrefsElement, (child) => {
            if(child.tagName == "INPUT") {
                child.addEventListener("change", updatePreferences);
                child.addEventListener("click", updatePreferences);
            }
            if(child.value && child.value.length != 0 && cookie.includes(child.value)) {
                child.checked = true;
            }
        });
        document.body.appendChild(topLevelPrefsElement);
        document.getElementById("closePrefs").addEventListener("click", () => {
            if(gear.classList.contains("isOpen")) {
                gear.classList.remove("isOpen");
            }
            topLevelPrefsElement.setAttribute("style", "display:none");
        });
    } else {
        // when the prefs box closes, we set style="display:none", so just
        // unset that
        topLevelPrefsElement.setAttribute("style", "");
    }
    // only add the isOpen class if we got this far without any errors
    if(!gear.classList.contains("isOpen")) {
        gear.classList.add("isOpen");
    }
}

function forEachChild(el, f) {
    f(el);
    if(el.children) {
        for(let child of el.children) {
            forEachChild(child, f);
        }
    }
}

applyPreferences();
