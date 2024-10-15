// @ts-check

import HtmlWebpackPlugin from 'html-webpack-plugin';
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';
import path from 'path';

/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack(config, { isServer, dev }) {
    // config.plugins = [
    //   ...config.plugins,
    //   new HtmlWebpackPlugin(),
    //   new WasmPackPlugin({
    //     crateDirectory: path.resolve(path.dirname("."), ".")
    //   })
    // ]
    config.experiments = { ...config.experiments, asyncWebAssembly: true, layers: true, topLevelAwait: true };

    return config;
  },
  reactStrictMode: true
}

export default nextConfig;
