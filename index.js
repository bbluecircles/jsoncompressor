import { decompress_json, compress_json } from './pkg';
import '@carbon/web-components/es/components/ui-shell/index.js';

class Utils {
    static downloadURL = (data, fileName) => {
        const a = document.createElement('a');
        a.href = data;
        a.download = fileName;
        document.body.appendChild(a);
        a.style.display = 'none';
        a.click();
        a.remove();
    }
}

class CompressPage extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }
    connectedCallback() {
        this.render();
    }
    render = () => {
        this.shadowRoot.innerHTML = /*html*/`
            <style>
                .page {
                    display: grid;
                    width: 100%;
                    min-height: 100vh;
                    grid-template-columns: minmax(auto, 1fr);
                    grid-template-rows: auto;
                    place-content: center;
                }

            </style>
            <main id="compress-json" class="page">
                <cds-header aria-label="IBM Platform Name">
                <cds-header-menu-button
                button-label-active="Close menu"
                button-label-inactive="Open menu"></cds-header-menu-button>
                <cds-header-name href="javascript:void 0" prefix="IBM"
                >[Platform]</cds-header-name
                >
                <cds-header-nav menu-bar-label="IBM [Platform]">
                <cds-header-nav-item href="javascript:void 0">Link 1</cds-header-nav-item>
                <cds-header-nav-item href="javascript:void 0">Link 2</cds-header-nav-item>
                <cds-header-nav-item href="javascript:void 0">Link 3</cds-header-nav-item>
                <cds-header-menu menu-label="Link 4" trigger-content="Link 4">
                    <cds-header-menu-item href="javascript:void 0"
                    >Sub-link 1</cds-header-menu-item
                    >
                    <cds-header-menu-item href="javascript:void 0"
                    >Sub-link 2</cds-header-menu-item
                    >
                    <cds-header-menu-item href="javascript:void 0"
                    >Sub-link 3</cds-header-menu-item
                    >
                </cds-header-menu>
                </cds-header-nav>
            </cds-header>
            <cds-side-nav aria-label="Side navigation" expanded>
                <cds-side-nav-items>
                <cds-side-nav-menu title="L0 menu">
                    <cds-side-nav-menu-item href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                    <cds-side-nav-menu-item href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                    <cds-side-nav-menu-item href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                </cds-side-nav-menu>
                <cds-side-nav-menu title="L0 menu">
                    <cds-side-nav-menu-item href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                    <cds-side-nav-menu-item active aria-current="page" href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                    <cds-side-nav-menu-item href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                </cds-side-nav-menu>
                <cds-side-nav-menu title="L0 menu">
                    <cds-side-nav-menu-item href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                    <cds-side-nav-menu-item href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                    <cds-side-nav-menu-item href="www.ibm.com">
                    L0 menu item
                    </cds-side-nav-menu-item>
                </cds-side-nav-menu>
                <cds-side-nav-divider></cds-side-nav-divider>
                <cds-side-nav-link href="javascript:void(0)">L0 link</cds-side-nav-link>
                <cds-side-nav-link href="javascript:void(0)">L0 link</cds-side-nav-link>
                </cds-side-nav-items>
            </cds-side-nav>
            </main>
        `;
    }
}
customElements.define('compress-page', CompressPage);

class App {
    constructor() {
        this.init();
    }
    setupHead = () => {
        const fontAwesomeScript = document.createElement('script');
        const basicStyles = document.createElement('style');
        basicStyles.innerHTML = `
            body {
                margin: 0;
                padding: 0;
                min-height: 100vh;
            }
        `;
        fontAwesomeScript.src = 'https://kit.fontawesome.com/f17e33830e.js';
        fontAwesomeScript.setAttribute('crossorigin', 'anonymous');
        document.head.appendChild(fontAwesomeScript);
        document.head.appendChild(basicStyles);
    }
    init = () => {
        this.setupHead();
        document.body.style.background = "rgb(60, 60, 60)";
        const page = document.createElement('compress-page');
        page.innerHTML = `
            <file-upload style="display: grid; place-content: center;"></file-upload>
        `;
        document.body.appendChild(page);
    }
}

const app = new App();