import { forwardRef } from 'react';
import PropTypes from 'prop-types';
import './Input.css';

/**
 * Input component for consistent input styling and behavior
 * @param {Object} props - Component props
 * @param {string} props.type - Input type (text, password, email, etc.)
 * @param {string} props.name - Input name (also used as id if id is not provided)
 * @param {Object} props.inputProps - Props to pass directly to the input element (value, onChange, placeholder, etc.)
 * @param {string} props.error - Error message
 * @param {string} props.label - Input label
 * @param {string} props.className - Additional CSS class
 * @param {React.Ref} ref - Forwarded ref
 * @returns {JSX.Element} - Input component
 */
const Input = forwardRef(({
  type = 'text',
  name,
  error,
  label,
  className = '',
  inputProps = {},
  ...rest
}, ref) => {
  const { required } = inputProps;
  const inputId = inputProps.id || name;
  const inputClasses = [
    'ui-input',
    error ? 'ui-input--error' : '',
    className
  ].filter(Boolean).join(' ');

  return (
    <div className="ui-input-container">
      {label && (
        <label htmlFor={inputId} className="ui-input-label">
          {label}
          {required && <span className="ui-input-required">*</span>}
        </label>
      )}
      <input
        ref={ref}
        type={type}
        name={name}
        id={inputId}
        className={inputClasses}
        aria-invalid={!!error}
        {...inputProps}
        {...rest}
      />
      {error && <div className="ui-input-error">{error}</div>}
    </div>
  );
});

Input.displayName = 'Input';

Input.propTypes = {
  type: PropTypes.string,
  name: PropTypes.string.isRequired,
  error: PropTypes.string,
  label: PropTypes.string,
  className: PropTypes.string,
  inputProps: PropTypes.object
};

export default Input;
