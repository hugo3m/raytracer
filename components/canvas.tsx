import Engine from "@/utils/engine";
import { useEffect, useRef } from "react"

type Props = {
    width: number,
    height: number,
}

export default function Canvas({width, height}: Props){

    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const run = async () => {
            const WASM = await import("wasm");
            const raytracer = new WASM.Raytracer(width, height);
            if (canvasRef.current){
                Engine.create(raytracer, canvasRef.current, width, height);
            }
        };
        run();
    }, []);

    return <canvas ref={canvasRef} width={width} height={height}></canvas>
}

export {type Props};