import React, { useState } from 'react';
import { useNavigate, useLocation, Link } from 'react-router-dom';
import { AuthService } from '../api/generated';

export const VerifyEmail: React.FC = () => {
    const location = useLocation();
    const navigate = useNavigate();
    const passedEmail = (location.state as { email?: string })?.email || '';

    const [email, setEmail] = useState(passedEmail);
    const [code, setCode] = useState('');
    const [error, setError] = useState('');
    const [success, setSuccess] = useState('');

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError('');
        setSuccess('');

        try {
            const res = await AuthService.verifyEmail({ email, code });
            setSuccess(res.message);
            // Redirect to login after a short delay
            setTimeout(() => navigate('/login'), 1500);
        } catch (err: any) {
            setError(err?.body?.message || 'Verification failed');
        }
    };

    return (
        <div style={{ maxWidth: '400px', margin: '40px auto', padding: '20px', border: '1px solid #ccc', borderRadius: '8px' }}>
            <h2>Verify Email</h2>
            <p style={{ color: '#666', marginBottom: '15px' }}>
                A verification code was sent to your email address. Enter it below to activate your account.
            </p>
            {error && <div style={{ color: 'red', marginBottom: '10px' }}>{error}</div>}
            {success && <div style={{ color: 'green', marginBottom: '10px' }}>{success}</div>}
            <form onSubmit={handleSubmit}>
                <div style={{ marginBottom: '15px' }}>
                    <label style={{ display: 'block', marginBottom: '5px' }}>Email:</label>
                    <input
                        type="email"
                        value={email}
                        onChange={e => setEmail(e.target.value)}
                        style={{ width: '100%', padding: '8px', boxSizing: 'border-box' }}
                        required
                    />
                </div>
                <div style={{ marginBottom: '15px' }}>
                    <label style={{ display: 'block', marginBottom: '5px' }}>Verification Code:</label>
                    <input
                        type="text"
                        value={code}
                        onChange={e => setCode(e.target.value)}
                        style={{ width: '100%', padding: '8px', boxSizing: 'border-box', letterSpacing: '4px', textAlign: 'center', fontSize: '18px' }}
                        maxLength={6}
                        placeholder="000000"
                        required
                    />
                </div>
                <button type="submit" style={{ width: '100%', padding: '10px', backgroundColor: '#8b5cf6', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
                    Verify
                </button>
            </form>
            <div style={{ marginTop: '15px', textAlign: 'center' }}>
                <Link to="/login">Back to Login</Link>
            </div>
        </div>
    );
};
