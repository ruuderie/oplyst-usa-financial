import type { Meta, StoryObj } from '@storybook/vue3';

import Tooltip from '../components/ui/tooltip/Tooltip.vue';

const meta = {
  title: 'Tooltip',
  component: Tooltip,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Tooltip>;

export default meta;
type Story = StoryObj<typeof Tooltip>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};