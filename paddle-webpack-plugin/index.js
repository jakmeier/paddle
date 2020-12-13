/**
 * A helper plugin to resolve import statements inside JS snippets.
 * It enables those snippets to call methods exported by Rust.
 * It even works when the export originates in a library crate and webpack is used on a dependent crate.
 * 
 * (Sidenote: This is a prototype. I hope there will be a better solution in the future.)
 */
class PaddleWebpackPlugin {
    constructor() {
        this.rust_home;
    }
    apply(compiler) {
        compiler.resolverFactory.hooks.resolver.for('normal').tap('name', resolver => {
            // Store the name of the module root file when it is loaded
            resolver.hooks.module.tapAsync('PaddleWebpackPlugin', (moduleInfo, _context, callback) => {
                this.rust_home = moduleInfo.descriptionFileData.module;
                callback();
            });
            // Handle paths marked with #RUST#
            resolver.hooks.resolve.tapAsync('PaddleWebpackPlugin', (request, context, callback) => {
                // wasm-bindgen snippets are copied to a folder like pkg/snippets/crate-name-0123456789abcdef/path/to/file/within/crate
                // if a module path used in a snippet contains #RUST# then it should be resolved from the module root (because that's where wasm-bindgen exports are placed)
                let index;
                if ((index = request.path.search("snippets")) > -1 && request.request.search("#RUST#") > -1) {
                    const moduleRoot = this.rust_home;
                    const updatedRequest = {
                        ...request,
                        path: request.path.substr(0, index),
                        request: request.request.replace('#RUST#', moduleRoot)
                    };

                    const target = resolver.ensureHook('parsedResolve');
                    return resolver.doResolve(target, updatedRequest, null, context, callback);
                } else {
                    callback();
                }
            });
        });
    }
}

module.exports = PaddleWebpackPlugin;