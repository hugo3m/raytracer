import { useEffect, useRef } from "react"

import { Raytracer } from "wasm";

type Props = {
    width: number,
    height: number,
}

export default function Canvas({width, height}: Props){

    const canvasRef = useRef<HTMLCanvasElement>(null);

    const update = (raytracer: Raytracer) => {
        if (!canvasRef.current) return;
        const canvas = canvasRef.current;
        const ctx = canvas.getContext("2d")
        if (!ctx) return;
        console.log('drawing!');
        ctx.putImageData(new ImageData(new Uint8ClampedArray(raytracer.draw()), width, height), 0, 0);
    };

    useEffect(() => {
        const run = async () => {
            const WASM = await import("wasm");
            const raytracer = new WASM.Raytracer(width, height);
            setInterval(update, 1000, raytracer);
        };
        run();
    });

    return <canvas ref={canvasRef} width={width} height={height}></canvas>
}

export {type Props};