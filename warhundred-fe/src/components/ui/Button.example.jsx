import Button from './Button';

/**
 * Example usage of the Button component
 */
const ButtonExample = () => {
  const handleClick = () => {
    alert('Button clicked!');
  };

  return (
    <div>
      <h2>Button Component Examples</h2>

      {/* Basic primary button */}
      <div style={{ marginBottom: '1rem' }}>
        <Button
          variant="primary"
          buttonProps={{ onClick: handleClick }}
        >
          Primary Button
        </Button>
      </div>

      {/* Secondary button with small size */}
      <div style={{ marginBottom: '1rem' }}>
        <Button
          variant="secondary"
          size="small"
          buttonProps={{ 
            onClick: handleClick,
            title: "This is a secondary button"
          }}
        >
          Small Secondary Button
        </Button>
      </div>

      {/* Danger button with disabled state */}
      <div style={{ marginBottom: '1rem' }}>
        <Button
          variant="danger"
          buttonProps={{ 
            disabled: true,
            'aria-label': "Dangerous action"
          }}
        >
          Disabled Danger Button
        </Button>
      </div>

      {/* Full width button */}
      <div style={{ marginBottom: '1rem' }}>
        <Button
          variant="primary"
          fullWidth
          buttonProps={{ 
            onClick: handleClick,
            type: "submit"
          }}
        >
          Full Width Submit Button
        </Button>
      </div>
    </div>
  );
};

export default ButtonExample;
