import type { Meta, StoryObj } from '@storybook/vue3';

import Popover from '../components/ui/popover/Popover.vue';

const meta = {
  title: 'Popover',
  component: Popover,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Popover>;

export default meta;
type Story = StoryObj<typeof Popover>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};