import { decompress_json, compress_json } from './pkg';

class CompressPage extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }
    connectedCallback() {
        this.render();
    }
    render = () => {
        this.shadowRoot.innerHTML = `
            <style>
                .page {
                    display: grid;
                    width: 100%;
                    min-height: 100vh;
                    grid-template-columns: minmax(auto, 1fr);
                    grid-template-rows: auto;
                    place-content: center;
                }
                #compress-json 
            </style>
            <main id="compress-json" class="page">
                <slot></slot>
            </main>
        `;
    }
}
customElements.define('compress-page', CompressPage);
class FileUpload extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.fileContents = '';
    }
    connectedCallback() {
        this.render();
    }
    render = () => {
        this.shadowRoot.innerHTML = `
            <style>
                @import url('https://ka-f.fontawesome.com/releases/v6.5.1/css/free.min.css?token=f17e33830e');

                .file-upload {
                    position: relative;
                    color: #ccc;
                    display: grid;
                    place-content: center;
                    width: 150px;
                    height: 150px;
                    cursor: pointer;
                    transition: 0.2s transform ease;
                }
                .file-upload #upload-json-input {
                    position: absolute;
                    width: 100%;
                    height: 100%;
                    cursor: pointer;
                    opacity: 0;
                }
                .file-upload:hover i {
                    color: rgb(136, 183, 225);
                }
                .file-upload:hover {
                    transform: translateY(-10px);
                }
                .file-upload i {
                    font-size: 5rem;
                    transition: 0.2s color ease;
                }
                .file-upload span {
                    position: absolute;
                }
                .file-upload span[pos="top-left"] {
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 25px;
                    height: 25px;
                    border-top: 10px solid #ccc;
                    border-left: 10px solid #ccc;
                    border-radius: 5px;

                }
                .file-upload span[pos="top-right"] {
                    position: absolute;
                    top: 0;
                    right: 0;
                    width: 25px;
                    height: 25px;
                    border-top: 10px solid #ccc;
                    border-right: 10px solid #ccc;
                    border-radius: 5px;
                }
                .file-upload span[pos="bottom-left"] {
                    position: absolute;
                    bottom: 0;
                    left: 0;
                    width: 25px;
                    height: 25px;
                    border-bottom: 10px solid #ccc;
                    border-left: 10px solid #ccc;
                    border-radius: 5px;

                }
                .file-upload span[pos="bottom-right"] {
                    position: absolute;
                    right: 0;
                    bottom: 0;
                    width: 25px;
                    height: 25px;
                    border-bottom: 10px solid #ccc;
                    border-right: 10px solid #ccc;
                    border-radius: 5px;
                }
            </style>
            <div class="file-upload">
                <span pos="top-left"></span><span pos="top-right"></span><span pos="bottom-left"></span><span pos="bottom-right"></span>
                <i class="fa-solid fa-upload"></i>
                <input type="file" id="upload-json-input" />
            </div>
        `
    }
}
customElements.define('file-upload', FileUpload);
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