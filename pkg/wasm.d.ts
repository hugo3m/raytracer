/* tslint:disable */
/* eslint-disable */
/**
*/
export class Raytracer {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @param {number} sphere_number
*/
  constructor(width: number, height: number, sphere_number: number);
/**
* @param {boolean} forward
* @param {boolean} backward
* @param {boolean} left
* @param {boolean} right
* @param {boolean} up
* @param {boolean} down
* @param {number} delta_time
*/
  input(forward: boolean, backward: boolean, left: boolean, right: boolean, up: boolean, down: boolean, delta_time: number): void;
/**
* @returns {Uint8Array}
*/
  draw(): Uint8Array;
}
