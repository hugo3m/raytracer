"use client";

import Canvas, {Props as CanvasProps} from "./canvas";
import { useEffect, useState } from "react";

import { Nullable } from "@/utils/type";

export default function Test() {

  // const [canvasProps, setCanvasProps] = useState<Nullable<CanvasProps>>(null);

  // const updateCanvasProps = () => {
  //   if (window !== undefined){
  //     const gap: number = 50;
  //     const maxSize: number = 600;
  //     const limit: number = Math.min(window.innerWidth, window.innerHeight);
  //     const limitIntToString: string = limit.toFixed(0).toString();
  //     const power: number = limitIntToString.length;
  //     const exponent: number = parseInt(limitIntToString[0]);
  //     const size: number = (exponent * (10 ** (power - 1))) - gap;
  //     setCanvasProps({width: Math.min(size, maxSize), height: Math.min(size, maxSize)});
  //   }
  // }

  // useEffect(() => {
  //   updateCanvasProps();
  // }, [])

  return (
    <div className="flex flex-1 justify-center p-1"><Canvas width={600} height={600}/></div>
  );
}
