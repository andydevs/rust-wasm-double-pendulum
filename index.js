console.log('Hello from index.js');

const root = document.createElement('div');
root.style.fontFamily = 'Arial, sans-serif';
root.style.padding = '24px';
root.innerHTML = `
  <h1>Rust WASM Double Pendulum</h1>
  <p>Webpack + HtmlWebpackPlugin demo. Replace this with your app entry.</p>
`;

document.body.appendChild(root);
