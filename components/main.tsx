"use client";

import Engine, { InputInfo } from "@/utils/engine";
import { useEffect, useRef, useState } from "react";

import Input from "./input";
import { Nullable } from "@/utils/type";
import Slider from '@mui/material/Slider';

export default function Main() {

  const canvasRef = useRef<HTMLCanvasElement>(null);

  const [fps, setFps] = useState<number>(0);
  const [inputInfo, setInputInfo] = useState<Nullable<InputInfo>>(null);
  const [pixels, setPixels] = useState<number>(600);
  const [sphereNumber, setSphereNumber] = useState<number>(3);

  useEffect(() => {
      const run = async () => {
          const WASM = await import("wasm");
          const raytracer = new WASM.Raytracer(pixels, pixels, sphereNumber);
          if (canvasRef.current){
              Engine.create(raytracer, canvasRef.current, pixels, pixels, setFps, setInputInfo);
          }
      };
      run();
      return () => {
          Engine.destroy();
      }
  }, [sphereNumber, pixels]);

  return (
  <div className="flex flex-1 flex-col">
    <div className="flex flex-row justify-center">
      <div className="flex flex-1 flex-col p-12">
          <span>Presentation:</span>
          <p>
              Homemade raytracer made by <a className="text-blue-700" href="https://github.com/hugo3m">hugo3m</a> inspired from
              <a className="text-blue-700" href="https://gabrielgambetta.com/computer-graphics-from-scratch/index.html"> Computer graphics from scratch, Gabriel Gambetta</a>.
                Built using Rust web assembly and NextJS.
          </p>
      </div>
      <div className="flex flex-2 flex-col max-w-[40vw] min-w-[40vw]">
        <span className="">{fps.toFixed(0)}FPS</span>
        <canvas ref={canvasRef} width={pixels} height={pixels}/>
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
            <div>
              <span>Parameters:</span>
              <div>
                  <span>Number of pixels width and height</span>
                  <Slider
                      value={pixels}
                      onChange={(event, value) => setPixels(value as number)}
                      marks
                      valueLabelDisplay="auto"
                      min={100}
                      step={100}
                      max={1000}
                  />
              </div>
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
    </div>
  );
}
