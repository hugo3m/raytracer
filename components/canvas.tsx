import Engine, { InputInfo } from "@/utils/engine";
import { Nullable } from "@/utils/type";
import { useEffect, useRef, useState } from "react";
import Input from "./input";
import Slider from '@mui/material/Slider';

type Props = {
    width: number,
    height: number,
}


export default function Canvas({width, height}: Props){

    const canvasRef = useRef<HTMLCanvasElement>(null);

    const [fps, setFps] = useState<number>(0);
    const [inputInfo, setInputInfo] = useState<Nullable<InputInfo>>(null);
    const [sphereNumber, setSphereNumber] = useState<number>(3);

    useEffect(() => {
        const run = async () => {
            const WASM = await import("wasm");
            const raytracer = new WASM.Raytracer(width, height, sphereNumber);
            if (canvasRef.current){
                Engine.create(raytracer, canvasRef.current, width, height, setFps, setInputInfo);
            }
        };
        run();
        return () => {
            Engine.destroy();
        }
    }, [sphereNumber]);

    return <div className="flex flex-1 flex-col">
        <div className="flex flex-row justify-center">
            <div className="flex flex-1 flex-col p-12">
                <p>
                    Homemade raytracer made by <a className="text-blue-700" href="https://github.com/hugo3m">hugo3m</a> inspired from
                    <a className="text-blue-700" href="https://gabrielgambetta.com/computer-graphics-from-scratch/index.html"> Computer graphics from scratch, Gabriel Gambetta</a>.
                     Built using Rust web assembly and NextJS.
                </p>
            </div>
            <div className="flex flex-2 flex-col">
                <span className="">{fps.toFixed(0)}FPS</span>
                <canvas ref={canvasRef} width={width} height={height}/>
                {inputInfo ? <div className="flex flex-col items-center">
                    <div className="flex flex-row">
                        <Input input="Q" isDown={inputInfo.up}/>
                        <Input input="W" isDown={inputInfo.forward}/>
                        <Input input="E" isDown={inputInfo.down}/>
                    </div>
                    <div className="flex flex-row">
                        <Input input="A" isDown={inputInfo.left}/>
                        <Input input="S" isDown={inputInfo.backward}/>
                        <Input input="D" isDown={inputInfo.right}/>
                    </div>
                    </div> : null}
            </div>
            <div className="flex flex-1 flex-col p-12">
                <span>Number of spheres</span>
                    <Slider
                        value={sphereNumber}
                        onChange={(event, value) => setSphereNumber(value as number)}
                        marks
                        valueLabelDisplay="auto"
                        min={1}
                        max={10}
                    />
                </div>
            </div>


        </div>
}

export {type Props};