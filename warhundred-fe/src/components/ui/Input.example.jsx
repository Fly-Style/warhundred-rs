import { useState } from 'react';
import Input from './Input';

/**
 * Example usage of the Input component
 */
const InputExample = () => {
  const [formData, setFormData] = useState({
    username: '',
    email: '',
    password: ''
  });

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  return (
    <div>
      <h2>Input Component Examples</h2>

      {/* Basic usage */}
      <Input
        type="text"
        name="username"
        label="Username"
        inputProps={{
          value: formData.username,
          onChange: handleChange,
          placeholder: "Enter your username",
          required: true
        }}
      />

      {/* With error message */}
      <Input
        type="email"
        name="email"
        label="Email"
        error={formData.email.includes('@') ? '' : 'Please enter a valid email'}
        inputProps={{
          value: formData.email,
          onChange: handleChange,
          placeholder: "Enter your email"
        }}
      />

      {/* Disabled input */}
      <Input
        type="password"
        name="password"
        label="Password"
        inputProps={{
          value: formData.password,
          onChange: handleChange,
          placeholder: "Enter your password",
          disabled: true
        }}
      />
    </div>
  );
};

export default InputExample;
