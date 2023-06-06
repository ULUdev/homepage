import {LitElement, html, css} from 'lit';
import {customElement, property, state, query} from 'lit/decorators.js';
import {styleMap} from 'lit/directives/style-map.js';

@customElement('hp-cookiebanner')
class CookieBanner extends LitElement {
    @property({ type: Boolean })
    clicked = localStorage.getItem('cookie_ok')==null?false:true;

    static styles = css`.banner {
        position: fixed;
bottom: 0;
width: 100%;
left: 0;
color: var(--fg);
background-color: var(--bg);
padding: 10px 10px;
align-items: center;
text-align: center;
justify-content: center
}
.banner button {
display: inline-block;
border: none;
color: var(--bg);
background-color: var(--fg);
padding: 10px 10px;
margin-left: 10px;
}
.banner button:hover {
cursor: pointer;
}
.banner p {
font-size: 15px;
display: inline-block;
}
`;

    @property()
    styles = { display: this.clicked?"none":"flex" };
    
    render() {
        if (!this.clicked) {
            return html`
<div class="banner" style=${styleMap(this.styles)}>
            <p>This page uses cookies to know if you are logged in or not</p>
<button style="float: right" @click="${() => {localStorage.setItem("cookie_ok", "clicked"); this.clicked = true;}}">Ok</button>
            </div>
            `;
        }
    }
}
