import { Raytracer } from "@/pkg/wasm";

enum Event {
    Up = 1,
    Down = 0,
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

class Engine {
    static #instance: Engine;

    canvas: HTMLCanvasElement;
    height: number;
    inputInfo: InputInfo;
    raytracer: Raytracer;
    refreshRate: number;
    width: number

    private constructor(raytracer: Raytracer, canvas: HTMLCanvasElement, width: number, height: number) {
        this.raytracer = raytracer;
        this.canvas = canvas;
        this.width = width;
        this.height = height;
        this.inputInfo = new InputInfo();
        this.refreshRate = 60;
        this.update();

        addEventListener("keydown", (event) => {
            this.handleEvent(event.key, Event.Down);
        });

        addEventListener("keyup", (event) => {
            this.handleEvent(event.key, Event.Up);
        });
    }

    public static create(raytracer: Raytracer, canvas: HTMLCanvasElement, width: number, height: number) {
        if (!Engine.#instance) {
            Engine.#instance = new Engine(raytracer, canvas, width, height);
        }
    }

    public static get instance(): Engine {
        return Engine.#instance;
    }

    private update() {
        const ctx = this.canvas.getContext("2d")
        if (!ctx) return;
        const startTime: number = Date.now();
        this.raytracer.input(this.inputInfo.forward,
            this.inputInfo.backward,
            this.inputInfo.left,
            this.inputInfo.right,
            this.inputInfo.up,
            this.inputInfo.down);
        ctx.putImageData(new ImageData(new Uint8ClampedArray(this.raytracer.draw()), this.width, this.height), 0, 0);
        const elapsedTime: number = Date.now() - startTime;
        const minTime = 1 / this.refreshRate;
        const timeToWait = elapsedTime - minTime;
        setTimeout(this.update.bind(this), timeToWait > 0 ? timeToWait : 0);
    };

    private mapEventToBoolean(event: Event): boolean {
        switch(event){
            case Event.Up:
                return true;
            case Event.Down:
                return false;
            default:
                return false;
        }
    }

    private handleEvent(key: string, event: Event){
        const eventBoolean = this.mapEventToBoolean(event);
        console.log(key, event);
        switch(key){
            case "w":
                this.inputInfo.forward = eventBoolean;
                break
            case "s":
                this.inputInfo.backward = eventBoolean;
                break;
            case "a":
                this.inputInfo.left = eventBoolean;
                break;
            case "d":
                this.inputInfo.right = eventBoolean;
                break;
            case "q":
                this.inputInfo.up = eventBoolean;
                break;
            case "e":
                this.inputInfo.down = eventBoolean;
                break;
            default:
                break;
        }
    }

}

export default Engine;