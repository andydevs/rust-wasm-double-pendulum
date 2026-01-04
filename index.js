function main(pkg) {
    const a = BigInt(Math.round(10 * Math.random()))
    const b = BigInt(Math.round(10 * Math.random()))
    const c = pkg.add(a, b)

    const root = document.createElement('div');
    root.style.fontFamily = 'Arial, sans-serif';
    root.style.padding = '24px';
    root.innerHTML = `
    <h1>Rust WASM Double Pendulum</h1>
    <p>Webpack + HtmlWebpackPlugin demo. Replace this with your app entry.</p>
    <p>${a} + ${b} = ${c}</p>
    `;

    document.body.appendChild(root);
}

require('./pkg').then(pkg => main(pkg))