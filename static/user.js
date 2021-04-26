let skills = document.getElementsByClassName("skill");
Array.from(skills).forEach(function(rect) {
    rect.setAttributeNS(null, "rx", "5");
    rect.setAttributeNS(null, "ry", "5");
});

const hasPositionChanged = ({ pos, prevPos }) => pos !== prevPos;

const valueInRange = ({ minScale, maxScale, scale }) => scale <= maxScale && scale >= minScale;

const getTranslate = ({ minScale, maxScale, scale }) => ({ pos, prevPos, translate }) =>
    valueInRange({ minScale, maxScale, scale }) && hasPositionChanged({ pos, prevPos })
        ? translate + (pos - prevPos * scale) * (1 - 1 / scale)
        : translate;

const getMatrix = ({ scale, translateX, translateY }) => `matrix(${scale}, 0, 0, ${scale}, ${translateX}, ${translateY})`;

const getScale = ({ scale, minScale, maxScale, scaleSensitivity, deltaScale }) => {
    let newScale = scale + (deltaScale / (scaleSensitivity / scale));
    newScale = Math.max(minScale, Math.min(newScale, maxScale));
    return [scale, newScale];
};

const pan = ({ state, originX, originY }) => {
    state.transformation.translateX += originX;
    state.transformation.translateY += originY;
    state.element.style.transform =
        getMatrix({ scale: state.transformation.scale, translateX: state.transformation.translateX, translateY: state.transformation.translateY });
};

const canPan = (state) => ({
    panBy: ({ originX, originY }) => pan({ state, originX, originY }),
    panTo: ({ originX, originY, scale }) => {
        state.transformation.scale = scale;
        pan({ state, originX: originX - state.transformation.translateX, originY: originY - state.transformation.translateY });
    },
});

const canZoom = (state) => ({
    zoom: ({ x, y, deltaScale }) => {
        const { left, top } = state.element.getBoundingClientRect();
        const { minScale, maxScale, scaleSensitivity } = state;
        const [scale, newScale] = getScale({ scale: state.transformation.scale, deltaScale, minScale, maxScale, scaleSensitivity });
        const originX = x - left;
        const originY = y - top;
        const newOriginX = originX / scale;
        const newOriginY = originY / scale;
        const translate = getTranslate({ scale, minScale, maxScale });
        const translateX = translate({ pos: originX, prevPos: state.transformation.originX, translate: state.transformation.translateX });
        const translateY = translate({ pos: originY, prevPos: state.transformation.originY, translate: state.transformation.translateY });

        state.element.style.transformOrigin = `${newOriginX}px ${newOriginY}px`;
        state.element.style.transform = getMatrix({ scale: newScale, translateX, translateY });
        state.transformation = { originX: newOriginX, originY: newOriginY, translateX, translateY, scale: newScale };
    }
});

const renderer = ({ minScale, maxScale, element, scaleSensitivity = 10 }) => {
    const state = {
        element,
        minScale,
        maxScale,
        scaleSensitivity,
        transformation: {
            originX: 0,
            originY: 0,
            translateX: 0,
            translateY: 0,
            scale: 1
        },
    };
    return Object.assign({}, canZoom(state), canPan(state));
};

let mousedown = false;

    const container = document.getElementById("tree");
    const instance = renderer({ scaleSensitivity: 50, minScale: .1, maxScale: 30, element: container });
    container.addEventListener("wheel", (event) => {
        event.preventDefault();
        instance.zoom({
            deltaScale: Math.sign(event.deltaY) > 0 ? -5 : 5,
            x: event.pageX,
            y: event.pageY
        });
    });
    container.addEventListener("dblclick", () => {
        instance.panTo({
            originX: 0,
            originY: 0,
            scale: 1,
        });
    });
    container.addEventListener("mousedown", _ => {
        mousedown = true;
    });
    container.addEventListener("mouseup", _ => {
        mousedown = false;
    });
    container.addEventListener("mousemove", (event) => {
        if (!mousedown) {
            return;
        }
        event.preventDefault();
        instance.panBy({
            originX: event.movementX,
            originY: event.movementY
        });
    })
