import React, { useState } from 'react';
import { Box, Button, Card, CardContent, TextField, Typography } from '@mui/material';

const InvestorPage: React.FC = () => {
    const [amount, setAmount] = useState('');

    // Placeholder для start_time (ви додасте реальну перевірку дати)
    const startTime = new Date('2025-09-01T00:00:00'); // Приклад дати
    const isClaimAvailable = new Date() >= startTime; // Логіка доступності (використовуйте реальний час)

    // Тут буде ваш onBuy handler
    const handleBuy = () => {
        // Логіка для покупки on-chain
    };

    // Тут буде ваш onClaim handler
    const handleClaim = () => {
        // Логіка для claim токенів on-chain
    };

    return (
        <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh', backgroundColor: '#f0f4f8' }}>
            <Card sx={{ maxWidth: 400, padding: 4, boxShadow: 3, borderRadius: 3 }}>
                <CardContent>
                    <Typography variant="h4" gutterBottom sx={{ textAlign: 'center', color: '#1976d2' }}>
                        Купити Токени
                    </Typography>
                    <TextField
                        fullWidth
                        type="number"
                        label="Сума (кількість токенів)"
                        variant="outlined"
                        value={amount}
                        onChange={(e) => setAmount(e.target.value)}
                        inputProps={{ min: 0 }}
                        sx={{ mb: 3 }}
                    />
                    <Button fullWidth variant="contained" color="primary" onClick={handleBuy} sx={{ mb: 2, py: 1.5, fontSize: '1rem' }}>
                        Купити
                    </Button>
                    <Typography variant="body2" sx={{ textAlign: 'center', mb: 2 }}>
                        Токени доступні для отримання після {startTime.toLocaleString()}
                    </Typography>
                    <Button fullWidth variant="contained" color="secondary" onClick={handleClaim} disabled={!isClaimAvailable} sx={{ py: 1.5, fontSize: '1rem' }}>
                        Отримати Токени
                    </Button>
                </CardContent>
            </Card>
        </Box>
    );
};

export default InvestorPage;