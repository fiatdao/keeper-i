import * as React from 'react'
import BigNumber from 'bignumber.js';

import './app.css';

const U256_MAX = '0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';

export function App(props) {
    const { positions, lastBlock } = props;

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
                        const tokenId = new BigNumber(position[1].token_id).toString();
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
