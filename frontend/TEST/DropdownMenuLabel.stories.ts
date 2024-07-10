import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuLabel from '../components/ui/dropdown-menu/DropdownMenuLabel.vue';

const meta = {
  title: 'DropdownMenuLabel',
  component: DropdownMenuLabel,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuLabel>;

export default meta;
type Story = StoryObj<typeof DropdownMenuLabel>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};