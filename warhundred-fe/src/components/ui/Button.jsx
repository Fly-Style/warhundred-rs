import PropTypes from 'prop-types';
import './Button.css';

/**
 * Button component for consistent button styling and behavior
 * @param {Object} props - Component props
 * @param {string} props.variant - Button variant (primary, secondary, danger)
 * @param {string} props.size - Button size (small, medium, large)
 * @param {boolean} props.fullWidth - Whether the button should take full width
 * @param {Object} props.buttonProps - Props to pass directly to the button element (onClick, disabled, etc.)
 * @param {React.ReactNode} props.children - Button content
 * @param {string} props.className - Additional CSS class
 * @returns {JSX.Element} - Button component
 */
const Button = ({
  variant = 'primary',
  size = 'medium',
  fullWidth = false,
  buttonProps = {},
  children,
  className = '',
  ...rest
}) => {

  const buttonClasses = [
    'ui-button',
    `ui-button--${variant}`,
    `ui-button--${size}`,
    fullWidth ? 'ui-button--full-width' : '',
    className
  ].filter(Boolean).join(' ');

  return (
    <button
      className={buttonClasses}
      {...buttonProps}
      {...rest}
    >
      {children}
    </button>
  );
};

Button.propTypes = {
  variant: PropTypes.oneOf(['primary', 'secondary', 'danger']),
  size: PropTypes.oneOf(['small', 'medium', 'large']),
  fullWidth: PropTypes.bool,
  buttonProps: PropTypes.object,
  children: PropTypes.node.isRequired,
  className: PropTypes.string
};

export default Button;
