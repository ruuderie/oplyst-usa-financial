import type { Meta, StoryObj } from '@storybook/vue3';

import Toggle from '../components/ui/toggle/Toggle.vue';

const meta = {
  title: 'Toggle',
  component: Toggle,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Toggle>;

export default meta;
type Story = StoryObj<typeof Toggle>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};