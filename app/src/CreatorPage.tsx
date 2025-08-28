import React, { useState } from 'react';
import {
  Box,
  Button,
  Card,
  CardContent,
  Checkbox,
  FormControlLabel,
  Grid,
  IconButton,
  TextField,
  Typography,
} from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import RemoveIcon from '@mui/icons-material/Remove';
import * as anchor from '@coral-xyz/anchor';
import { Program, web3, BN, AnchorError } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import type { TokenLaunchpad } from '../../target/types/token_launchpad';
import { useWallet } from '@solana/wallet-adapter-react';
import { TOKEN_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/utils/token';
import { ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Buffer } from 'buffer'
window.Buffer = Buffer

interface Creator {
  address: string;
  verified: boolean;
  share: number;
}

interface FormToken {
  name: string;
  symbol: string;
  uri: string;
  icon: File | null;
  decimals: number;
  total_supply: number;
  authority_freeze: boolean;
  recipient_payer: boolean;
  creators: Creator[];
  seller_fee_basis_points: number;
}

const CreatorPage: React.FC<{}> = () => {
  const TOKEN_METADATA_PROGRAM_ID = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');
  const { publicKey } = useWallet();
  const [creators, setCreators] = useState<Creator[]>([{ address: '', verified: false, share: 0 }]);
  const [formToken, setFormToken] = useState<FormToken>({
    name: '',
    symbol: '',
    uri: '',
    icon: null,
    decimals: 0,
    total_supply: 0,
    authority_freeze: false,
    recipient_payer: true,
    creators: [],
    seller_fee_basis_points: 0,
  });

  const addCreator = () => setCreators([...creators, { address: '', verified: false, share: 0 }]);
  const removeCreator = (index: number) => setCreators(creators.filter((_, i) => i !== index));

  const handleCreatorChange = (index: number, field: 'address' | 'verified' | 'share', value: string) => {
    const updatedCreators = [...creators];
    if (field === 'share') updatedCreators[index][field] = Number(value);
    else if (field === 'verified') updatedCreators[index][field] = value === 'true';
    else updatedCreators[index][field] = value;
    setCreators(updatedCreators);
    setFormToken({ ...formToken, creators: updatedCreators });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!publicKey) {
      alert('Please connect your wallet first!');
      return;
    }
    const metadata = { name: formToken.name, symbol: formToken.symbol, image: formToken.icon };
    const jsonBlob = new Blob([JSON.stringify(metadata, null, 2)], { type: 'application/json' });
    const jsonUrl = URL.createObjectURL(jsonBlob);
    setFormToken({ ...formToken, uri: jsonUrl });

    const provider = anchor.AnchorProvider.env();    anchor.setProvider(provider);
    const program = anchor.workspace.TokenLaunchpad as Program<TokenLaunchpad>;

    try {
      const [mintPda] = PublicKey.findProgramAddressSync([Buffer.from('mint'), publicKey.toBuffer()], program.programId);
      const [mintAuthPda] = PublicKey.findProgramAddressSync([Buffer.from('mint-auth'), publicKey.toBuffer()], program.programId);
      const [metadataPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('metadata'), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintPda.toBuffer()],
        TOKEN_METADATA_PROGRAM_ID
      );

      const args = {
        name: formToken.name || 'MyToken',
        symbol: formToken.symbol || 'MTK',
        uri: formToken.uri || 'https://example.com/metadata.json',
        decimals: formToken.decimals || 6,
        totalSupply: new BN(formToken.total_supply || 1000000),
        authorityFreeze: publicKey,
        recipientPayer: formToken.recipient_payer || false,
        creators: formToken.creators.length > 0
          ? formToken.creators.map(c => ({ address: new PublicKey(c.address || publicKey.toBase58()), verified: c.verified, share: c.share }))
          : null,
        sellerFeeBasisPoints: formToken.seller_fee_basis_points || null,
      };

      const accounts = {
        payer: publicKey,
        mint: mintPda,
        mintAuth: mintAuthPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        metadata: metadataPda,
      };

      const tx = await program.methods.createCustomMint(args).accounts(accounts).rpc();
      console.log('Transaction signature:', tx);
      alert('Mint created successfully! Signature: ' + tx);
    } catch (error) {
      alert("Hello");
    }
  };

  return (
    <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'flex-start', minHeight: '50vh', paddingTop: 40, backgroundColor: '#f0f4f8' }}>
      <Card sx={{ width: '500px', maxWidth: '90vw', padding: 4, boxShadow: 3, borderRadius: 3 }}>
        <CardContent>
          <Typography variant="h4" gutterBottom sx={{ textAlign: 'center', color: '#1976d2' }}>Створити Токен</Typography>
          <form onSubmit={handleSubmit}>
            <Grid container spacing={2}>
              <Grid size={12}><TextField fullWidth label="Назва токена" variant="outlined" value={formToken.name} onChange={(e) => setFormToken({ ...formToken, name: e.target.value })} /></Grid>
              <Grid size={12}><TextField fullWidth label="Символ" variant="outlined" value={formToken.symbol} onChange={(e) => setFormToken({ ...formToken, symbol: e.target.value })} /></Grid>
              <Grid size={12}><TextField fullWidth label="URI (метадані)" variant="outlined" value={formToken.uri} onChange={(e) => setFormToken({ ...formToken, uri: e.target.value })} /></Grid>
              <Grid size={12}><TextField fullWidth type="number" label="Decimals" variant="outlined" inputProps={{ min: 0 }} value={formToken.decimals} onChange={(e) => setFormToken({ ...formToken, decimals: Number(e.target.value) })} /></Grid>
              <Grid size={12}><TextField fullWidth type="number" label="Total Supply" variant="outlined" inputProps={{ min: 0 }} value={formToken.total_supply} onChange={(e) => setFormToken({ ...formToken, total_supply: Number(e.target.value) })} /></Grid>
              <Grid size={12}><TextField fullWidth type="number" label="Seller Fee Basis Points" variant="outlined" inputProps={{ min: 0, max: 10000 }} value={formToken.seller_fee_basis_points} onChange={(e) => setFormToken({ ...formToken, seller_fee_basis_points: Number(e.target.value) })} /></Grid>
              <Grid size={12}><FormControlLabel control={<Checkbox />} label="Authority Freeze" checked={formToken.authority_freeze} onChange={(_, checked) => setFormToken({ ...formToken, authority_freeze: checked })} /></Grid>
              <Grid size={12}><FormControlLabel control={<Checkbox />} label="Recipient Payer" checked={formToken.recipient_payer} onChange={(_, checked) => setFormToken({ ...formToken, recipient_payer: checked })} /></Grid>
              <Grid size={12}>
                <Typography variant="subtitle1" gutterBottom>Креатори</Typography>
                {creators.map((creator, index) => (
                  <Grid container spacing={2} key={index} alignItems="center">
                    <Grid size={8}><TextField fullWidth label="Адреса" variant="outlined" value={creator.address} onChange={(e) => handleCreatorChange(index, 'address', e.target.value)} /></Grid>
                    <Grid size={3}><TextField fullWidth type="number" label="Частка (%)" variant="outlined" value={creator.share} onChange={(e) => handleCreatorChange(index, 'share', e.target.value)} inputProps={{ min: 0, max: 100 }} /></Grid>
                    <Grid size={1}><IconButton onClick={() => removeCreator(index)} color="error"><RemoveIcon /></IconButton></Grid>
                  </Grid>
                ))}
                <Button startIcon={<AddIcon />} onClick={addCreator} sx={{ mt: 1 }}>Додати креатора</Button>
              </Grid>
              <Grid size={12}>
                <Typography variant="subtitle1" gutterBottom>Іконка</Typography>
                <input type="file" accept="image/*" onChange={(e) => setFormToken({ ...formToken, icon: e.target.files && e.target.files[0] ? e.target.files[0] : null })} />
              </Grid>
              <Grid size={12}><Button fullWidth variant="contained" color="primary" type="submit" sx={{ mt: 2, py: 1.5, fontSize: '1rem' }}>Створити Токен</Button></Grid>
            </Grid>
          </form>
        </CardContent>
      </Card>
    </Box>
  );
};

export default CreatorPage;