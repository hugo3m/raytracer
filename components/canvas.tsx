import { useEffect, useRef } from "react"


type Props = {
    width: number,
    height: number,

}

export default function Canvas({width, height}: Props){

    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const run = async () => {
            if (!canvasRef.current) return;
            const canvas = canvasRef.current;
            const ctx = canvas.getContext("2d")
            if (!ctx) return;
            const WASM = await import("wasm");
            const wasmData: Uint8Array = WASM.draw(width, height);
            ctx.putImageData(new ImageData(new Uint8ClampedArray(wasmData), width, height), 0, 0);
        };
        run();
    });

    return <canvas ref={canvasRef} width={width} height={height}></canvas>
}