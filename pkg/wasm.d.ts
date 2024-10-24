/* tslint:disable */
/* eslint-disable */
/**
*/
export class Raytracer {
  free(): void;
/**
* @param {number} width
* @param {number} height
*/
  constructor(width: number, height: number);
/**
* @param {boolean} forward
* @param {boolean} backward
* @param {boolean} left
* @param {boolean} right
* @param {boolean} up
* @param {boolean} down
*/
  input(forward: boolean, backward: boolean, left: boolean, right: boolean, up: boolean, down: boolean): void;
/**
* @returns {Uint8Array}
*/
  draw(): Uint8Array;
}
