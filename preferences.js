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
    if(cookie.includes("minweight300")) {
        forceElement("minWeight", "minweight300.css");
    } else if(cookie.includes("minweight200")) {
        forceElement("minWeight", "maxweight200.css");
    } else {
        forceElement("minWeight", null);
    }
    if(cookie.includes("maxweight700")) {
        forceElement("maxWeight", "maxweight700.css");
    } else if(cookie.includes("maxweight800")) {
        forceElement("maxWeight", "maxweight800.css");
    } else {
        forceElement("maxWeight", null);
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
    if((e = document.querySelector(`input[name="minWeight"]:checked`)?.value)) {
        elements.push(e);
    }
    if((e = document.querySelector(`input[name="maxWeight"]:checked`)?.value)) {
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
<!--<p>Minimum Font Weight:</p>
<ul>
<li><input type="radio" name="minWeight" id="minweight100" value="" checked> <label for="minweight100" style="font-weight: 100;">Min. weight: 100</label> (default)</li>
<li><input type="radio" name="minWeight" id="minweight200" value="weight200"> <label for="minweight200" style="font-weight: 200;">Min. weight: 200</label></li>
<li><input type="radio" name="minWeight" id="minweight300" value="weight300"> <label for="minweight300" style="font-weight: 300;">Min. weight: 300</label></li>
</ul>
<p>Maximum Font Weight:</p>
<ul>
<li><input type="radio" name="maxWeight" id="maxweight700" value="weight700"> <label for="maxweight700" style="font-weight: 700;">Max. weight: 700</label></li>
<li><input type="radio" name="maxWeight" id="maxweight800" value="weight800"> <label for="maxweight800" style="font-weight: 800;">Max. weight: 800</label></li>
<li><input type="radio" name="maxWeight" id="maxweight900" value="" checked> <label for="maxweight900" style="font-weight: 900;">Max. weight: 900</label> (default)</li>
</ul>
<p>(I think the only thing that goes past 700 is my book on modes, and only a few times at that.)-->
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
