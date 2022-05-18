import * as React from 'react'
import ReactDOM from 'react-dom'
import BigNumber from 'BigNumber.js';

import './app.css';

const U256_MAX = '0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';

function App() {
    const [positions, setPositions] = React.useState([]);
    const [lastBlock, setLastBlock] = React.useState(0);

    React.useEffect(() => {
        console.log(positions, lastBlock);
        if (positions.length == 0 || lastBlock == 0) {
            try {
                (async () => {
                    const response = await fetch('http://localhost:9000/');
                    const state = await response.json();
                    const lastBlock = state['last_block'];
                    const positions = state['cached_positions'].sort((a, b) => {
                        // @ts-ignore
                        if (new BigNumber(a[1].health_factor).lt(b[1].health_factor)) return -1;
                        // @ts-ignore
                        if (new BigNumber(a[1].health_factor).gt(b[1].health_factor)) return 1;
                        return 0;
                    });
                    setPositions(positions);
                    setLastBlock(lastBlock);
                })();
            } catch (error) { console.error(error); }
        }
      }, []);

    return (
        <div>
            <h1>Positions (block: {lastBlock})</h1>
            <table>
                <thead>
                    <tr>
                        <td>Vault</td>
                        <td>TokenId</td>
                        <td>Owner</td>
                        <td>Collateral Value (USD)</td>
                        <td>FIAT Debt</td>
                        <td>Health Factor</td>
                    </tr>
                </thead>
                <tbody>
                {
                    positions.map((position) => {
                        // @ts-ignore
                        const vault = position[1].vault;
                        // @ts-ignore
                        const tokenId = new BigNumber(position[1].token_id).toFormat();
                        // @ts-ignore
                        const owner = position[1].owner;
                        // @ts-ignore
                        const collateralValue = new BigNumber(position[1].collateral_value);
                        // @ts-ignore
                        const debt = new BigNumber(position[1].debt);
                        // @ts-ignore
                        const healthFactor = new BigNumber(position[1].health_factor);
                        
                        return (
                            <tr key={vault + tokenId + owner}>
                                <td>{vault}</td>
                                <td>{tokenId}</td>
                                <td>{owner}</td>
                                <td>{collateralValue.shiftedBy(-18).toFormat()}</td>
                                <td>{debt.shiftedBy(-18).toFormat()}</td>
                                <td>{(healthFactor.eq(new BigNumber(U256_MAX))) ? '∞' : healthFactor.shiftedBy(-18).toFormat()}</td>
                            </tr>
                        );
                    })
                }
                </tbody>
            </table>
        </div>
        
    )
}

ReactDOM.render(<App />, document.querySelector('#root'))
