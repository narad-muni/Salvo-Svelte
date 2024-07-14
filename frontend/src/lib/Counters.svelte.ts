let count = $state(0);

export const increment = () => {
  count += 1;
};

export const get = () => count;
