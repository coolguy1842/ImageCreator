const { invoke } = window.__TAURI__.tauri;
const { dialog } = window.__TAURI__;

const options = {
    path: null,
    datapack_path: "",
    simplify: {
        enabled: false,
        colors: 64
    },
    downscale: {
        enabled: false,
        width: 0,
        height: 0
    },
    scale: 1,
    quality: 4
};

var ImageSelectorEl;
var ImageDisplayEl;

var OptionsContainerEl;

var DownscaleContainerEl;
var DownscaleToggleEl;
var DownscaleWidthEl;
var DownscaleHeightEl;

var SimplifyContainerEl;
var SimplifyToggleEl;
var SimplifyColorsEl;

var UpdateImageEl;

var MinecraftScaleEl;
var MinecraftQualityEl;

var CreateDatapackEl;

var ImageMainContainerEl;

var GeneratingDatapackTextEl;
var LoadingImageTextEl;

var imageData = {
    data: null,
    width: null,
    height: null,
    original_width: null,
    original_height: null
};

async function load_image() {
    LoadingImageTextEl.css("display", "inherit");
    ImageMainContainerEl.css("pointerEvents", "none");

    imageData = await invoke("change_image", { options: options });
    if(imageData.data == "") {    
        OptionsContainerEl.attr("style", "display: none !important");
    }
    else {
        OptionsContainerEl.attr("style", "display: inherit !important");
    }

    ImageDisplayEl.attr("src", imageData.data);
    ImageDisplayEl.css("width", `${imageData.width}px`);
    ImageDisplayEl.css("height", `${imageData.height}px`);
    ImageDisplayEl.css("transform", `scale(${options.scale})`)

    LoadingImageTextEl.css("display", "none");
    ImageMainContainerEl.css("pointerEvents", "all");
}

async function change_image() {
    options.path = await dialog.open({
        multiple: false,
        directory: false,
        filters: [{
            name: 'Image',
            extensions: ["png","gif","jpeg","jpg","jfif","bmp","tiff","ico","webp","avif","pnm","pbm","pgm","ppm","pam","dds","dxt1","dxt3","dxt5","tga","openexr","farbfeld"]
        }]
    });

    if(options.path == null) {
        ImageDisplayEl.attr("src", "");
        OptionsContainerEl.attr("style", "display: none !important");
        
        return;
    }
    
    load_image();
}

window.addEventListener("DOMContentLoaded", () => {
    GeneratingDatapackTextEl = $("#generating-datapack-text");
    LoadingImageTextEl = $("#loading-image-text");


    ImageMainContainerEl = $("#image-main-container");
    OptionsContainerEl = $("#options-container");

    ImageDisplayEl = $("#image-selector-display"); 

    ImageSelectorEl = $("#image-selector-button");
    ImageSelectorEl.on("click", () => change_image());


    UpdateImageEl = $("#update-image-button");
    UpdateImageEl.on("click", () => load_image());

    
    DownscaleContainerEl = $("#downscale-container");
    DownscaleToggleEl = $("#downscale-toggle");
    DownscaleWidthEl = $("#downscale-width");
    DownscaleHeightEl = $("#downscale-height");

    DownscaleToggleEl.on("change", e => {
        options.downscale.enabled = e.target.checked;

        DownscaleContainerEl.attr("style", `display: ${options.downscale.enabled ? "inherit !important" : "none !important"}`);
    })

    DownscaleWidthEl.on("input", e => {
        if(e.target.value.match(/[^0-9]/)) {
            e.preventDefault();

            e.target.value = `${options.downscale.width > 0 ? options.downscale.width : ""}`;
            return;
        }

        options.downscale.width = Number(e.target.value);

        if(options.downscale.width > imageData.original_width) {
            e.target.value = imageData.original_width;
        }
    });
    
    DownscaleHeightEl.on("input", e => {
        if(e.target.value.match(/[^0-9]/)) {
            e.preventDefault();

            e.target.value = `${options.downscale.height > 0 ? options.downscale.height : ""}`;
            return;
        }

        options.downscale.height = Number(e.target.value);

        if(options.downscale.height > imageData.original_height) {
            e.target.value = imageData.original_height;
        }
    });

    
    SimplifyContainerEl = $("#simplify-container");
    SimplifyToggleEl = $("#simplify-toggle");
    SimplifyColorsEl = $("#simplify-colors");

    SimplifyToggleEl.on("change", e => {
        options.simplify.enabled = e.target.checked;

        SimplifyContainerEl.attr("style", `display: ${options.simplify.enabled ? "inherit !important" : "none !important"}`);
    })

    SimplifyColorsEl.on("input", e => {
        if(e.target.value.match(/[^0-9]/)) {
            e.preventDefault();

            e.target.value = `${options.simplify.colors > 0 ? options.simplify.colors : 64}`;
            return;
        }

        options.simplify.colors = Number(e.target.value);

        if(options.simplify.colors > 255) {
            e.target.value = 255;
        }
    });


    MinecraftScaleEl = $("#minecraft-scale");
    
    MinecraftScaleEl.on("input", e => {
        if(isNaN(Number(e.target.value))) {
            e.preventDefault();

            e.target.value = `${options.scale > 0 ? options.scale : 1}`;
            return;
        }

        options.scale = Number(e.target.value);
    });

    MinecraftScaleEl.on("change", e => {
        if(e.target.value.length <= 0) {
            e.target.value = 1;
            options.scale = 1;
        }
    });


    MinecraftQualityEl = $("#minecraft-quality");
    
    MinecraftQualityEl.on("input", e => {
        if(e.target.value.match(/[^0-9]/)) {
            e.preventDefault();

            e.target.value = `${options.quality > 0 ? options.quality : 4}`;
            return;
        }

        options.quality = Number(e.target.value);
    });

    MinecraftQualityEl.on("change", e => {
        if(e.target.value.length <= 0) {
            e.target.value = 4;
            options.quality = 4;
        }
    });

    CreateDatapackEl = $("#create-datapack-button");

    CreateDatapackEl.on("click", async e => {
        options.datapack_path = await dialog.open({
            multiple: false,
            directory: true,
        });

        if(options.datapack_path == null) {
            options.datapack_path = "";
            return;
        }
        
        GeneratingDatapackTextEl.css("display", "inherit");
        ImageMainContainerEl.css("pointerEvents", "none");
        
        await invoke("create_datapack", { options: options });

        GeneratingDatapackTextEl.css("display", "none");
        ImageMainContainerEl.css("pointerEvents", "all");
    });
});
