/**
 *
 * @returns {import("@babel/core").PluginObj}
 */
export default function () {
  return {
    visitor: {
      Identifier(path) {
        const name = path.node.name;
        // reverse the name: JavaScript -> tpircSavaJ
        path.node.name = name.split("").reverse().join("");

        return path;
      },
    },
  };
}
