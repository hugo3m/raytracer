import Engine from "@/utils/engine";
import { useEffect, useRef, useState } from "react"

type Props = {
    width: number,
    height: number,
}

export default function Canvas({width, height}: Props){

    const canvasRef = useRef<HTMLCanvasElement>(null);

    const [fps, setFps] = useState<number>(0);

    useEffect(() => {
        const run = async () => {
            const WASM = await import("wasm");
            const raytracer = new WASM.Raytracer(width, height);
            if (canvasRef.current){
                Engine.create(raytracer, canvasRef.current, width, height, setFps);
            }
        };
        run();
    }, []);

    return <div>
        <span>{fps.toFixed(0)}FPS</span>
        <canvas ref={canvasRef} width={width} height={height}/>
        </div>
}

export {type Props};