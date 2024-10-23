import { useEffect, useRef } from "react"

import { Raytracer } from "wasm";

type Props = {
    width: number,
    height: number,
}

class InputInfo{

    forward: boolean
    backward: boolean
    up: boolean
    down: boolean
    left: boolean
    right: boolean

    constructor(){
        this.forward = false;
        this.backward = false;
        this.down = false;
        this.up = false;
        this.left = false;
        this.right = false
    }
}

export default function Canvas({width, height}: Props){

    const canvasRef = useRef<HTMLCanvasElement>(null);

    const inputInfo = new InputInfo();

    addEventListener("keydown", (event) => {
        switch(event.key){
            case "w":
                inputInfo.forward = true;
                break
            case "s":
                inputInfo.backward = true;
                break;
            default:
                break;
        }
    });

    addEventListener("keyup", (event) => {
        switch(event.key){
            case "w":
                inputInfo.forward = false;
                break
            case "s":
                inputInfo.backward = false;
                break;
            default:
                break;
        }
    });

    const update = (raytracer: Raytracer) => {
        if (!canvasRef.current) return;
        const canvas = canvasRef.current;
        const ctx = canvas.getContext("2d")
        if (!ctx) return;
        raytracer.input(inputInfo.forward, inputInfo.backward, inputInfo.left, inputInfo.right, inputInfo.up, inputInfo.down);
        ctx.putImageData(new ImageData(new Uint8ClampedArray(raytracer.draw()), width, height), 0, 0);
        setTimeout(update, 125, raytracer);
    };

    useEffect(() => {
        const run = async () => {
            const WASM = await import("wasm");
            const raytracer = new WASM.Raytracer(width, height);
            setTimeout(update, 1000, raytracer);
        };
        run();
    });

    return <canvas ref={canvasRef} width={width} height={height}></canvas>
}

export {type Props};